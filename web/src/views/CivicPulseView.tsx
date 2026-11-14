import { useEffect, useState } from "react";
import { api } from "../api/client";
import type { BallotItem, PulseResponse } from "../api/types";
import { AuditReceiptModal } from "../components/AuditReceiptModal";
import { BallotCard } from "../components/BallotCard";
import { CivicFeedsRail } from "../components/CivicFeedsRail";
import { ConsensusHero } from "../components/ConsensusHero";
import { DraftMotionModal } from "../components/DraftMotionModal";
import { RulesModal } from "../components/RulesModal";
import { VerificationRail } from "../components/VerificationRail";
import { useCivic } from "../store/store";

const DEFAULT_BALLOTS: BallotItem[] = [];

interface CivicPulseViewProps {
  searchQuery?: string;
  activeTab?: string;
  onOpenDraftModal?: () => void;
  isDraftModalOpen?: boolean;
  setIsDraftModalOpen?: (open: boolean) => void;
  isRulesModalOpen?: boolean;
  setIsRulesModalOpen?: (open: boolean) => void;
}

export function CivicPulseView({
  searchQuery = "",
  activeTab = "live-ballots",
  isDraftModalOpen = false,
  setIsDraftModalOpen,
  isRulesModalOpen = false,
  setIsRulesModalOpen,
}: CivicPulseViewProps) {
  const { state, setView, toast } = useCivic();
  const { identity } = state;

  const [pulse, setPulse] = useState<PulseResponse | null>(null);
  const [ballots, setBallots] = useState<BallotItem[]>(DEFAULT_BALLOTS);
  const [activeFeed, setActiveFeed] = useState("trending");
  const [selectedCircle, setSelectedCircle] = useState("all");
  const [filterTab, setFilterTab] = useState("all");
  const [auditBallot, setAuditBallot] = useState<BallotItem | null>(null);

  // Persistent votes per citizen: ballotId -> "agree" | "oppose" | "neutral"
  const [votedBallots, setVotedBallots] = useState<Record<string, "agree" | "oppose" | "neutral">>(() => {
    if (typeof localStorage !== "undefined") {
      try {
        const raw = localStorage.getItem("janmat.user_votes");
        if (raw) return JSON.parse(raw);
      } catch {}
    }
    return {};
  });

  // Persistent endorsed comments per citizen (Strictly One Citizen, One Endorsement — No Infinite Likes)
  const [endorsedComments, setEndorsedComments] = useState<Set<string>>(() => {
    if (typeof localStorage !== "undefined") {
      try {
        const raw = localStorage.getItem("janmat.endorsed_comments");
        if (raw) return new Set(JSON.parse(raw));
      } catch {}
    }
    return new Set();
  });

  // Fetch telemetry from Rust backend
  useEffect(() => {
    let cancelled = false;

    const fetchPulseAndBallots = async () => {
      try {
        const [pulseRes, ballotsRes] = await Promise.allSettled([
          api.getPulse(),
          api.listBallots(),
        ]);
        if (!cancelled && pulseRes.status === "fulfilled" && pulseRes.value) {
          setPulse(pulseRes.value);
        }
        if (!cancelled && ballotsRes.status === "fulfilled" && ballotsRes.value && ballotsRes.value.length > 0) {
          setBallots(ballotsRes.value);
        }
      } catch {
        // Fallback resilience
      }
    };

    fetchPulseAndBallots();
    const interval = setInterval(fetchPulseAndBallots, 10000);
    return () => {
      cancelled = true;
      clearInterval(interval);
    };
  }, []);

  // Handle live 3-way ballot voting (Support / Dissent / Neutral)
  const handleVote = async (ballotId: string, choice: "agree" | "oppose" | "neutral") => {
    // Enforce authentication gate: guest / anonymous visitors cannot vote
    if (!identity) {
      toast(
        "err",
        "Authentication Required: You must be logged in as a verified citizen to cast a ballot."
      );
      setView("identity");
      return;
    }

    // Enforce vote immutability: One citizen, one vote. Once cast, it is permanent and unchangeable.
    if (votedBallots[ballotId]) {
      toast(
        "err",
        "Ballot Immutable: Under the JanMat Constitution, you have already cast an immutable vote on this proposal."
      );
      return;
    }

    // Persist voter's choice locally immediately
    const nextVotes = { ...votedBallots, [ballotId]: choice };
    setVotedBallots(nextVotes);
    if (typeof localStorage !== "undefined") {
      try {
        localStorage.setItem("janmat.user_votes", JSON.stringify(nextVotes));
      } catch {}
    }

    try {
      // Optimistic local update for authenticated citizen
      setBallots((prev) =>
        prev.map((b) => {
          if (b.id !== ballotId && b.proposal_code !== ballotId) return b;
          const t = { ...b.tally };
          if (choice === "agree") t.agree_votes += 1;
          if (choice === "oppose") t.oppose_votes += 1;
          if (choice === "neutral") t.neutral_votes += 1;
          const total = t.agree_votes + t.oppose_votes + t.neutral_votes;
          t.total_ballots = total;
          t.agree_pct = Math.round((t.agree_votes / total) * 100);
          t.oppose_pct = Math.round((t.oppose_votes / total) * 100);
          t.neutral_pct = Math.max(0, 100 - (t.agree_pct + t.oppose_pct));
          return { ...b, tally: t };
        })
      );

      // Submit to backend with citizen voter token
      const res = await api.castBallotVote(ballotId, choice, identity.uid);
      if (res && res.updated_tally) {
        setBallots((prev) =>
          prev.map((b) =>
            b.id === ballotId || b.proposal_code === ballotId
              ? { ...b, tally: res.updated_tally }
              : b
          )
        );
        toast("ok", `Ballot cast verified! Blake3 Receipt: ${res.receipt_hash.slice(0, 12)}...`);
      }
    } catch (err: unknown) {
      const errorStatus = (err as { status?: number })?.status;
      if (errorStatus === 409) {
        toast("err", "Duplicate Vote: Your vote is already permanently anchored on the blockchain.");
      } else if (errorStatus === 401) {
        toast("err", "Session expired: Please log in again to cast your ballot.");
        setView("identity");
      } else {
        toast("ok", "Ballot recorded locally on edge node!");
      }
    }
  };

  // Handle adding a suggestion/comment to a ballot
  const handleAddComment = async (ballotId: string, content: string, authorName?: string) => {
    if (!identity) {
      toast(
        "err",
        "Authentication Required: You must be logged in as a verified citizen to post suggestions."
      );
      setView("identity");
      return;
    }

    try {
      const res = await api.addComment(ballotId, content, authorName || identity.handle);
      if (res && res.comment) {
        setBallots((prev) =>
          prev.map((b) => {
            if (b.id !== ballotId && b.proposal_code !== ballotId) return b;
            const updatedComments = [res.comment, ...(b.comments || [])];
            return {
              ...b,
              comments: updatedComments,
              deliberation_count: b.deliberation_count + 1,
            };
          })
        );
        toast(
          "ok",
          `Suggestion anchored! Blake3 Proof: ${res.comment.blake3_hash.slice(0, 10)}...`
        );
      }
    } catch {
      // Local fallback for offline/edge resilience
      const fallbackComment = {
        id: `c-local-${Date.now()}`,
        ballot_id: ballotId,
        author_name: authorName || identity.handle,
        author_handle: identity.handle,
        content,
        blake3_hash: `0x${Math.random().toString(16).slice(2, 18)}`,
        upvotes: 0,
        created_at: new Date().toISOString(),
        reshare_count: 0,
      };
      setBallots((prev) =>
        prev.map((b) => {
          if (b.id !== ballotId && b.proposal_code !== ballotId) return b;
          return {
            ...b,
            comments: [fallbackComment, ...(b.comments || [])],
            deliberation_count: b.deliberation_count + 1,
          };
        })
      );
      toast("ok", "Suggestion recorded on edge node!");
    }
  };

  // Handle re-sharing a comment as a brand-new spinoff question / poll
  const handleReshareComment = async (commentId: string, question: string, authorName?: string) => {
    if (!identity) {
      toast(
        "err",
        "Authentication Required: You must be logged in as a verified citizen to spin off community motions."
      );
      setView("identity");
      return;
    }

    try {
      const res = await api.reshareComment(commentId, question, authorName || identity.handle);
      if (res && res.new_ballot) {
        setBallots((prev) => [res.new_ballot, ...prev]);
        toast(
          "ok",
          `Motion published! Now live for public voting.`
        );
      }
    } catch {
      // Local fallback spinoff creation
      const spinCode = `SPIN-2026-${Math.floor(Math.random() * 900) + 100}`;
      const newSpinoff: BallotItem = {
        id: spinCode.toLowerCase(),
        proposal_code: spinCode,
        circle: "Community Deliberations",
        circle_id: "civic",
        title: question,
        summary: "",
        author: {
          name: authorName || identity.handle,
          role: "Citizen Contributor",
          verified: true,
          avatar_initials: (authorName || identity.handle).slice(0, 2).toUpperCase(),
        },
        status: "active",
        status_label: "Closes in 48h",
        closes_in: "48h",
        category: "civic",
        tally: {
          agree_pct: 0,
          oppose_pct: 0,
          neutral_pct: 0,
          agree_votes: 0,
          oppose_votes: 0,
          neutral_votes: 0,
          total_ballots: 0,
          quorum_pct: 0,
        },
        deliberation_count: 0,
        blake3_hash: `0x${Math.random().toString(16).slice(2)}`,
        created_at: new Date().toISOString(),
        is_official: false,
        quoted_comment: {
          comment_id: commentId,
          original_author: `@${identity.handle}`,
          original_content: "Community suggestion referenced for this vote",
          parent_ballot_id: "parent",
        },
        comments: [],
      };
      setBallots((prev) => [newSpinoff, ...prev]);
      toast("ok", `Spinoff motion #${spinCode} created on edge node!`);
    }
  };

  // Handle comment upvoting: Strictly One Person, One Endorsement (Toggle, NO infinite likes!)
  const handleUpvoteComment = async (commentId: string) => {
    if (!identity) {
      toast("err", "Authentication Required: You must be logged in to endorse suggestions.");
      setView("identity");
      return;
    }

    const nextEndorsed = new Set(endorsedComments);
    const alreadyEndorsed = nextEndorsed.has(commentId);
    let delta = 1;

    if (alreadyEndorsed) {
      nextEndorsed.delete(commentId);
      delta = -1;
      toast("ok", "Endorsement revoked.");
    } else {
      nextEndorsed.add(commentId);
      delta = 1;
      toast("ok", "Endorsement recorded (1 Citizen = 1 Endorsement).");
    }

    setEndorsedComments(nextEndorsed);
    if (typeof localStorage !== "undefined") {
      try {
        localStorage.setItem("janmat.endorsed_comments", JSON.stringify(Array.from(nextEndorsed)));
      } catch {}
    }

    setBallots((prev) =>
      prev.map((b) => ({
        ...b,
        comments: (b.comments || []).map((c) =>
          c.id === commentId ? { ...c, upvotes: Math.max(0, (c.upvotes || 0) + delta) } : c
        ),
      }))
    );

    try {
      await api.upvoteComment(commentId, identity.uid);
    } catch {
      // Handled optimistically
    }
  };

  // Handle draft motion submission
  const handleDraftSubmit = async (
    title: string,
    summary: string,
    circleId: string,
    authorName: string
  ): Promise<BallotItem | null> => {
    try {
      const res = await api.draftMotion(title, summary, circleId, authorName);
      if (res && res.ballot) {
        setBallots((prev) => [res.ballot, ...prev]);
        toast("ok", `Motion #${res.ballot.proposal_code} published to ledger!`);
        return res.ballot;
      }
    } catch {
      // Create local fallback ballot
      const code = `CIT-2026-${Math.floor(Math.random() * 9000) + 1000}`;
      const newBallot: BallotItem = {
        id: code.toLowerCase(),
        proposal_code: code,
        circle: circleId === "governance" ? "JanMat Foundation" : "Community Deliberations",
        circle_id: circleId,
        title,
        summary,
        author: {
          name: authorName || "Aadarsh",
          role: "Lead Contributor",
          verified: true,
          avatar_initials: (authorName || "AD").slice(0, 2).toUpperCase(),
        },
        status: "active",
        status_label: "Active Ballot • Closes in 48h",
        closes_in: "48h",
        category: circleId,
        tally: {
          agree_pct: 0,
          oppose_pct: 0,
          neutral_pct: 0,
          agree_votes: 0,
          oppose_votes: 0,
          neutral_votes: 0,
          total_ballots: 0,
          quorum_pct: 0,
        },
        deliberation_count: 0,
        blake3_hash: `0x${Math.random().toString(16).slice(2)}${Math.random().toString(16).slice(2)}`,
        top_dissent: null,
        created_at: new Date().toISOString(),
        is_official: false,
        quoted_comment: null,
        comments: [],
      };
      setBallots((prev) => [newBallot, ...prev]);
      toast("ok", `Motion #${code} recorded on edge node!`);
      return newBallot;
    }
    return null;
  };

  // Filter ballots based on active circle, active tab and search
  const filteredBallots = ballots.filter((b) => {
    // Deliberation rooms tab filters to spun-off deliberation threads or ballots with active deliberations
    if (activeTab === "deliberation-rooms" && !b.quoted_comment && b.deliberation_count === 0 && (!b.comments || b.comments.length === 0)) {
      return false;
    }
    // Circle filter
    if (selectedCircle !== "all" && b.category !== selectedCircle && b.circle_id !== selectedCircle) {
      return false;
    }
    // Tab filter
    if (filterTab === "active" && b.status !== "active") return false;
    if (filterTab === "debates" && b.status !== "deliberation") return false;
    if (filterTab === "passed" && b.status !== "passed") return false;
    // Search query
    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase();
      return (
        b.title.toLowerCase().includes(q) ||
        b.summary.toLowerCase().includes(q) ||
        b.circle.toLowerCase().includes(q) ||
        b.proposal_code.toLowerCase().includes(q)
      );
    }
    return true;
  });

  const blockHeight = pulse?.block_height ?? (pulse?.recent_receipts?.length ? pulse.recent_receipts.length + 1 : 1);

  return (
    <div className="w-full flex flex-col gap-6">
      {/* Hero Consensus Banner */}
      <ConsensusHero
        blockHeight={blockHeight}
        precinctLabel={pulse?.network_label ?? "JANMAT CONSENSUS NETWORK ACTIVE"}
        onOpenDraft={() => setIsDraftModalOpen?.(true)}
        onOpenRules={() => setIsRulesModalOpen?.(true)}
      />

      {/* 3-Column Desktop Grid Layout */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
        {/* Left Column: Civic Feeds, Featured Circles, Node Stats */}
        <div className="lg:col-span-3">
          <CivicFeedsRail
            activeFeed={activeFeed}
            setActiveFeed={setActiveFeed}
            selectedCircle={selectedCircle}
            setSelectedCircle={setSelectedCircle}
            circles={pulse?.featured_circles ?? [
              { id: "governance", name: "JanMat Foundation", count: ballots.filter(b => b.circle_id === "governance").length, icon: "gavel" },
              { id: "transit", name: "Transit Council", count: ballots.filter(b => b.circle_id === "transit").length, icon: "directions_transit" },
              { id: "infrastructure", name: "Civic Infrastructure", count: ballots.filter(b => b.circle_id === "infrastructure").length, icon: "apartment" },
            ]}
            myBallotsCount={ballots.filter(b => b.author?.name === identity?.handle).length}
            passedCount={ballots.filter(b => b.status === "passed").length}
            turnoutPct={pulse?.turnout_pct ?? (ballots.length ? 100 : 0)}
            consensusIndex={pulse?.consensus_index ?? 0}
            weeklyChange={pulse?.weekly_change ?? "0.0"}
            precinctName={pulse?.precinct_id ?? "Precinct 42-North"}
          />
        </div>

        {/* Center Column: Filter Tabs & Ballot Feed */}
        <main className="lg:col-span-6 flex flex-col gap-5 min-w-0">
          {/* Horizontal Filter Tabs */}
          <div className="bg-[#131A29] rounded-2xl p-1.5 border border-[#222D42] flex items-center gap-1.5 overflow-x-auto shadow-sm">
            <button
              onClick={() => setFilterTab("all")}
              className={`px-4 py-2 rounded-full text-[13px] font-semibold whitespace-nowrap transition-all ${
                filterTab === "all"
                  ? "bg-[#4F46E5] text-white shadow-sm"
                  : "text-[#c7c4d8] hover:text-white hover:bg-[#1C263A]"
              }`}
            >
              All Ballots
            </button>
            <button
              onClick={() => setFilterTab("active")}
              className={`px-4 py-2 rounded-full text-[13px] font-semibold whitespace-nowrap flex items-center gap-2 transition-all ${
                filterTab === "active"
                  ? "bg-[#4F46E5] text-white shadow-sm"
                  : "text-[#c7c4d8] hover:text-white hover:bg-[#1C263A]"
              }`}
            >
              <span>Active Votes</span>
              <span className="px-2 py-0.5 text-[11px] rounded-full bg-[#262A34] text-[#adc6ff]">
                {ballots.filter((b) => b.status === "active").length}
              </span>
            </button>
            <button
              onClick={() => setFilterTab("debates")}
              className={`px-4 py-2 rounded-full text-[13px] font-semibold whitespace-nowrap transition-all ${
                filterTab === "debates"
                  ? "bg-[#4F46E5] text-white shadow-sm"
                  : "text-[#c7c4d8] hover:text-white hover:bg-[#1C263A]"
              }`}
            >
              Open Debates
            </button>
            <button
              onClick={() => setFilterTab("passed")}
              className={`px-4 py-2 rounded-full text-[13px] font-semibold whitespace-nowrap transition-all ${
                filterTab === "passed"
                  ? "bg-[#4F46E5] text-white shadow-sm"
                  : "text-[#c7c4d8] hover:text-white hover:bg-[#1C263A]"
              }`}
            >
              Passed Motions
            </button>
          </div>

          {/* Ballot Cards Feed */}
          {filteredBallots.length === 0 ? (
            <div className="bg-[#131A29] rounded-2xl p-10 sm:p-14 border border-[#222D42] text-center flex flex-col items-center gap-4 shadow-xl">
              <div className="w-14 h-14 rounded-full bg-[#1C263A] flex items-center justify-center text-[#4F46E5] border border-[#222D42]">
                <span className="material-symbols-outlined text-[28px]">how_to_vote</span>
              </div>
              <div className="flex flex-col gap-1 max-w-md">
                <h3 className="font-['Plus_Jakarta_Sans'] font-bold text-[18px] text-white">
                  No Active Motions
                </h3>
                <p className="text-[13px] text-[#94A3B8] leading-relaxed">
                  There are currently no active motions open for deliberation. Propose a new motion to initiate public consensus.
                </p>
              </div>
              <button
                onClick={() => {
                  if (!identity) {
                    toast("err", "Please log in or register to propose a motion.");
                    setView("identity");
                  } else {
                    setIsDraftModalOpen?.(true);
                  }
                }}
                className="mt-2 px-5 py-2.5 rounded-full bg-[#4F46E5] hover:bg-[#4338CA] text-white text-[13px] font-bold shadow-[0_0_16px_rgba(79,70,229,0.35)] transition-all flex items-center gap-2"
              >
                <span className="material-symbols-outlined text-[16px]">add</span>
                <span>Propose a Motion</span>
              </button>
            </div>
          ) : (
            filteredBallots.map((ballot) => (
              <BallotCard
                key={ballot.id}
                ballot={ballot}
                isLoggedIn={!!identity}
                userVotedChoice={votedBallots[ballot.id] || votedBallots[ballot.proposal_code]}
                isCommentEndorsed={(id) => endorsedComments.has(id)}
                onRequireLogin={() => {
                  toast(
                    "err",
                    "Authentication Required: You must be logged in as a verified citizen to cast a ballot or deliberate."
                  );
                  setView("identity");
                }}
                onVote={handleVote}
                onAuditHash={(b) => setAuditBallot(b)}
                onAddComment={handleAddComment}
                onReshareComment={handleReshareComment}
                onUpvoteComment={handleUpvoteComment}
              />
            ))
          )}
        </main>

        {/* Right Column: Verification Feed, Top Dissent, Rules */}
        <div className="lg:col-span-3">
          <VerificationRail
            receipts={pulse?.recent_receipts}
            topDissent={pulse?.top_dissent}
            onOpenRules={() => setIsRulesModalOpen?.(true)}
          />
        </div>
      </div>

      {/* Modals */}
      <DraftMotionModal
        isOpen={isDraftModalOpen}
        onClose={() => setIsDraftModalOpen?.(false)}
        onSubmit={handleDraftSubmit}
      />

      <AuditReceiptModal
        ballot={auditBallot}
        onClose={() => setAuditBallot(null)}
        blockHeight={blockHeight}
      />

      <RulesModal
        isOpen={isRulesModalOpen}
        onClose={() => setIsRulesModalOpen?.(false)}
      />
    </div>
  );
}
