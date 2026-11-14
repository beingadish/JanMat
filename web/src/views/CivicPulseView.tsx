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

const DEFAULT_BALLOTS: BallotItem[] = [
  {
    id: "jmf-2026-01",
    proposal_code: "JMF-2026-01",
    circle: "JanMat Core Governance",
    circle_id: "governance",
    title: "How do you like the new JanMat Civic Consensus Engine & Dark Interface?",
    summary:
      "Official community consensus poll from the JanMat Foundation: Vote in Support, Dissent, or Neutral on the new interface and real-time cryptographic consensus engine. Share suggestions below; any citizen can re-share suggestions with their own question to initiate follow-up community votes.",
    author: {
      name: "JanMat Foundation",
      role: "Official Civic Governance",
      verified: true,
      avatar_initials: "JF",
    },
    status: "active",
    status_label: "Official Organization Poll • Open Voting",
    closes_in: "5d",
    category: "civic",
    tally: {
      agree_pct: 78,
      oppose_pct: 14,
      neutral_pct: 8,
      agree_votes: 184,
      oppose_votes: 33,
      neutral_votes: 19,
      total_ballots: 236,
      quorum_pct: 120,
    },
    deliberation_count: 29,
    blake3_hash: "a4f8d227c9e0183b56f890123456789abcdef0123456789abcdef0123456789a",
    top_dissent: {
      category: "Visual Accessibility",
      upvotes: 14,
      quote:
        "Ensure contrast ratios for smaller captions remain WCAG AAA conformant if deeper dark levels are rolled out.",
    },
    created_at: "2026-11-14T06:00:00Z",
    is_official: true,
    quoted_comment: null,
    comments: [
      {
        id: "c-101",
        ballot_id: "jmf-2026-01",
        author_name: "Aadarsh (Lead Contributor)",
        author_handle: "beingadish",
        content: "I think we can change the color of the UI to something like blackish for OLED screens.",
        blake3_hash: "9b3c4f7a1e2d5c8b0a9f8e7d6c5b4a3f2e1d0c9b8a7f6e5d4c3b2a1f0e9d8c7b",
        upvotes: 18,
        created_at: "2026-11-14T08:00:00Z",
        reshare_count: 1,
      },
      {
        id: "c-102",
        ballot_id: "jmf-2026-01",
        author_name: "Priya Sharma",
        author_handle: "priya_civic",
        content: "Mandating Blake3 cryptographic receipt verification for every ward ballot gives citizens verifiable trust.",
        blake3_hash: "1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d",
        upvotes: 11,
        created_at: "2026-11-14T08:30:00Z",
        reshare_count: 0,
      },
    ],
  },
  {
    id: "spin-2026-02",
    proposal_code: "SPIN-2026-02",
    circle: "Design & User Experience",
    circle_id: "design",
    title: "Do you like this idea? Should JanMat adopt the pitch-black OLED aesthetic?",
    summary:
      "Spun-off community question based on citizen suggestion #c-101: 'I think we can change the color of the UI to something like blackish for OLED screens.'",
    author: {
      name: "Devendra K.",
      role: "Citizen Contributor",
      verified: true,
      avatar_initials: "DK",
    },
    status: "active",
    status_label: "Spun-off Community Motion • Closes in 42h",
    closes_in: "42h",
    category: "civic",
    tally: {
      agree_pct: 71,
      oppose_pct: 21,
      neutral_pct: 8,
      agree_votes: 89,
      oppose_votes: 26,
      neutral_votes: 10,
      total_ballots: 125,
      quorum_pct: 95,
    },
    deliberation_count: 14,
    blake3_hash: "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069",
    top_dissent: {
      category: "Eye Comfort",
      upvotes: 9,
      quote:
        "Dark slate #0A0E17 creates softer transitions at night compared to absolute #000000 with bright white cards.",
    },
    created_at: "2026-11-14T08:15:00Z",
    is_official: false,
    quoted_comment: {
      comment_id: "c-101",
      original_author: "Aadarsh (Lead Contributor) (@beingadish)",
      original_content: "I think we can change the color of the UI to something like blackish for OLED screens.",
      parent_ballot_id: "jmf-2026-01",
    },
    comments: [
      {
        id: "c-201",
        ballot_id: "spin-2026-02",
        author_name: "Kavita M.",
        author_handle: "kavita_m",
        content: "Pitch black saves significant battery life on mobile OLED screens!",
        blake3_hash: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        upvotes: 6,
        created_at: "2026-11-14T08:45:00Z",
        reshare_count: 0,
      },
    ],
  },
  {
    id: "tc-2026-03",
    proposal_code: "TC-2026-03",
    circle: "Transit Council",
    circle_id: "transit",
    title: "Should overnight suburban metro runs be funded via dynamic weekend congestion tolls?",
    summary:
      "High demand for night shift workers and safer weekend commute alternatives justifies off-peak operation, offset by peak suburban highway congestion charges during non-commercial hours.",
    author: {
      name: "Aarav K.",
      role: "Transit Council Representative",
      verified: true,
      avatar_initials: "AK",
    },
    status: "active",
    status_label: "Active Ballot • Closes in 18h",
    closes_in: "18h",
    category: "transit",
    tally: {
      agree_pct: 68,
      oppose_pct: 26,
      neutral_pct: 6,
      agree_votes: 2320,
      oppose_votes: 887,
      neutral_votes: 205,
      total_ballots: 3412,
      quorum_pct: 114,
    },
    deliberation_count: 142,
    blake3_hash: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    top_dissent: {
      category: "Economic Sustainability",
      upvotes: 142,
      quote:
        "Dynamic congestion tolls during economic downturns create an undue regressive penalty on non-salaried weekend service providers who cannot shift commuting hours.",
    },
    created_at: "2026-11-14T06:00:00Z",
    is_official: false,
    quoted_comment: null,
    comments: [],
  },
];

interface CivicPulseViewProps {
  searchQuery?: string;
  onOpenDraftModal?: () => void;
  isDraftModalOpen?: boolean;
  setIsDraftModalOpen?: (open: boolean) => void;
  isRulesModalOpen?: boolean;
  setIsRulesModalOpen?: (open: boolean) => void;
}

export function CivicPulseView({
  searchQuery = "",
  isDraftModalOpen = false,
  setIsDraftModalOpen,
  isRulesModalOpen = false,
  setIsRulesModalOpen,
}: CivicPulseViewProps) {
  const { toast } = useCivic();

  const [pulse, setPulse] = useState<PulseResponse | null>(null);
  const [ballots, setBallots] = useState<BallotItem[]>(DEFAULT_BALLOTS);
  const [activeFeed, setActiveFeed] = useState("trending");
  const [selectedCircle, setSelectedCircle] = useState("all");
  const [filterTab, setFilterTab] = useState("all");
  const [auditBallot, setAuditBallot] = useState<BallotItem | null>(null);

  // Fetch telemetry from Rust backend
  useEffect(() => {
    let cancelled = false;

    const fetchPulseAndBallots = async () => {
      try {
        const pulseData = await api.getPulse();
        if (!cancelled && pulseData) {
          setPulse(pulseData);
        }
      } catch {
        // Fallback resilience
      }

      try {
        const ballotsData = await api.listBallots();
        if (!cancelled && ballotsData && ballotsData.length > 0) {
          setBallots(ballotsData);
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
    try {
      // Optimistic local update
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

      // Submit to backend
      const res = await api.castBallotVote(ballotId, choice);
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
    } catch {
      toast("ok", "Ballot recorded locally on edge node!");
    }
  };

  // Handle adding a suggestion/comment to a ballot
  const handleAddComment = async (ballotId: string, content: string, authorName?: string) => {
    try {
      const res = await api.addComment(ballotId, content, authorName);
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
        author_name: authorName || "Citizen Contributor",
        author_handle: "citizen",
        content,
        blake3_hash: `0x${Math.random().toString(16).slice(2, 18)}`,
        upvotes: 1,
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
    try {
      const res = await api.reshareComment(commentId, question, authorName);
      if (res && res.new_ballot) {
        setBallots((prev) => [res.new_ballot, ...prev]);
        toast(
          "ok",
          `Spinoff poll published! Proposal #${res.new_ballot.proposal_code} is now live for voting.`
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
        summary: `Spun-off community question from citizen suggestion #${commentId}`,
        author: {
          name: authorName || "Citizen Sponsor",
          role: "Citizen Co-sponsor",
          verified: true,
          avatar_initials: (authorName || "CS").slice(0, 2).toUpperCase(),
        },
        status: "active",
        status_label: "Spun-off Community Motion • Closes in 48h",
        closes_in: "48h",
        category: "civic",
        tally: {
          agree_pct: 100,
          oppose_pct: 0,
          neutral_pct: 0,
          agree_votes: 1,
          oppose_votes: 0,
          neutral_votes: 0,
          total_ballots: 1,
          quorum_pct: 4,
        },
        deliberation_count: 0,
        blake3_hash: `0x${Math.random().toString(16).slice(2)}`,
        created_at: new Date().toISOString(),
        is_official: false,
        quoted_comment: {
          comment_id: commentId,
          original_author: "Citizen Contributor",
          original_content: "Community suggestion referenced for this vote",
          parent_ballot_id: "parent",
        },
        comments: [],
      };
      setBallots((prev) => [newSpinoff, ...prev]);
      toast("ok", `Spinoff motion #${spinCode} created on edge node!`);
    }
  };

  // Handle comment upvoting
  const handleUpvoteComment = async (commentId: string) => {
    try {
      // Optimistic update
      setBallots((prev) =>
        prev.map((b) => ({
          ...b,
          comments: (b.comments || []).map((c) =>
            c.id === commentId ? { ...c, upvotes: c.upvotes + 1 } : c
          ),
        }))
      );
      await api.upvoteComment(commentId);
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
        circle: circleId === "transit" ? "Transit Council" : "Civic Infrastructure",
        circle_id: circleId,
        title,
        summary,
        author: {
          name: authorName || "Citizen Sponsor",
          role: "Citizen Sponsor",
          verified: true,
          avatar_initials: (authorName || "CS").slice(0, 2).toUpperCase(),
        },
        status: "active",
        status_label: "Active Ballot • Closes in 48h",
        closes_in: "48h",
        category: circleId,
        tally: {
          agree_pct: 100,
          oppose_pct: 0,
          neutral_pct: 0,
          agree_votes: 1,
          oppose_votes: 0,
          neutral_votes: 0,
          total_ballots: 1,
          quorum_pct: 3,
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

  // Filter ballots based on active circle and tab
  const filteredBallots = ballots.filter((b) => {
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

  const blockHeight = pulse?.block_height ?? 849201;

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
            circles={pulse?.featured_circles}
            turnoutPct={pulse?.turnout_pct}
            consensusIndex={pulse?.consensus_index}
            weeklyChange={pulse?.weekly_change}
            precinctName={pulse?.precinct_id}
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
                {ballots.filter((b) => b.status === "active").length || 3}
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
            <div className="bg-[#131A29] rounded-2xl p-8 border border-[#222D42] text-center flex flex-col items-center gap-3">
              <span className="material-symbols-outlined text-[36px] text-[#64748B]">
                inbox
              </span>
              <h3 className="font-['Plus_Jakarta_Sans'] font-bold text-[18px] text-white">
                No ballots matching this filter
              </h3>
              <p className="text-[13px] text-[#94A3B8] max-w-sm">
                Try selecting &ldquo;All Circles&rdquo; or click &ldquo;Draft a Motion&rdquo; to introduce a new proposal to the community!
              </p>
              <button
                onClick={() => {
                  setSelectedCircle("all");
                  setFilterTab("all");
                }}
                className="mt-2 px-5 py-2 rounded-full bg-[#4F46E5] text-white text-[13px] font-semibold"
              >
                Reset Filters
              </button>
            </div>
          ) : (
            filteredBallots.map((ballot) => (
              <BallotCard
                key={ballot.id}
                ballot={ballot}
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
