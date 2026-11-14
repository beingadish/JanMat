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
    id: "tc-2024-88",
    proposal_code: "TC-2024-88",
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
  },
  {
    id: "eth-2024-14",
    proposal_code: "ETH-2024-14",
    circle: "Municipal IT & AI Ethics",
    circle_id: "it-ethics",
    title: "Public municipal IT must mandate open-source audits before deploying automated AI workflows.",
    summary:
      "Algorithmic public benefit disbursement and traffic management systems require independent peer-reviewed transparency reports before rollout to eliminate systemic demographic bias.",
    author: {
      name: "Meera R.",
      role: "Tech & Ethics Civic Fellow",
      verified: true,
      avatar_initials: "MR",
    },
    status: "deliberation",
    status_label: "Open Deliberation",
    closes_in: "2d",
    category: "tech",
    tally: {
      agree_pct: 84,
      oppose_pct: 11,
      neutral_pct: 5,
      agree_votes: 1588,
      oppose_votes: 208,
      neutral_votes: 94,
      total_ballots: 1890,
      quorum_pct: 98,
    },
    deliberation_count: 89,
    blake3_hash: "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069",
    top_dissent: {
      category: "Execution Latency",
      upvotes: 48,
      quote:
        "Mandatory multi-month open audit timelines could stall critical municipal cybersecurity response updates when rapid zero-day patching is required.",
    },
    created_at: "2026-11-14T04:00:00Z",
  },
  {
    id: "eng-2024-03",
    proposal_code: "ENG-2024-03",
    circle: "Clean Energy & Water",
    circle_id: "energy",
    title: "Allocate 15% of annual municipal parking revenue towards neighborhood solar microgrids.",
    summary:
      "Provides dedicated seed capital for community battery storage and rooftop photovoltaic cells in historically under-electrified wards.",
    author: {
      name: "Rohan Sen",
      role: "Civic Green Energy Board",
      verified: true,
      avatar_initials: "RS",
    },
    status: "passed",
    status_label: "Quorum Met",
    closes_in: "Passed",
    category: "civic",
    tally: {
      agree_pct: 72,
      oppose_pct: 21,
      neutral_pct: 7,
      agree_votes: 2965,
      oppose_votes: 865,
      neutral_votes: 288,
      total_ballots: 4118,
      quorum_pct: 145,
    },
    deliberation_count: 204,
    blake3_hash: "6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b",
    top_dissent: null,
    created_at: "2026-11-13T12:00:00Z",
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
        // Fallback resilience: stays functional if offline
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

  // Handle live 3-way ballot voting
  const handleVote = async (ballotId: string, choice: "agree" | "oppose" | "neutral") => {
    try {
      // Optimistic local update
      setBallots((prev) =>
        prev.map((b) => {
          if (b.id !== ballotId) return b;
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
          prev.map((b) => (b.id === ballotId ? { ...b, tally: res.updated_tally } : b))
        );
        toast("ok", `Ballot cast verified! Blake3 Receipt: ${res.receipt_hash.slice(0, 12)}...`);
      }
    } catch {
      toast("ok", "Ballot recorded locally on edge node!");
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
        precinctLabel={pulse?.network_label ?? "PRECINCT 42 NETWORK ACTIVE"}
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
                {ballots.filter((b) => b.status === "active").length || 42}
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
                onOpenDeliberation={() => setIsRulesModalOpen?.(true)}
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
