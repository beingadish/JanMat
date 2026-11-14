import { useState } from "react";
import type { BallotItem } from "../api/types";

interface BallotCardProps {
  ballot: BallotItem;
  onVote: (ballotId: string, choice: "agree" | "oppose" | "neutral") => Promise<void>;
  onAuditHash: (ballot: BallotItem) => void;
  onOpenDeliberation?: (ballot: BallotItem) => void;
}

export function BallotCard({
  ballot,
  onVote,
  onAuditHash,
  onOpenDeliberation,
}: BallotCardProps) {
  const [voting, setVoting] = useState(false);
  const [votedChoice, setVotedChoice] = useState<string | null>(null);

  const handleVoteClick = async (choice: "agree" | "oppose" | "neutral") => {
    if (voting) return;
    setVoting(true);
    try {
      await onVote(ballot.id, choice);
      setVotedChoice(choice);
    } finally {
      setVoting(false);
    }
  };

  const { tally } = ballot;

  return (
    <article className="bg-[#131A29] rounded-2xl p-5 sm:p-6 border border-[#222D42] shadow-md flex flex-col gap-4 transition-all hover:border-[#3E4C6D] hover:shadow-xl">
      {/* Card Header Meta */}
      <div className="flex items-start justify-between gap-3 flex-wrap sm:flex-nowrap">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-full bg-[#0566D9] flex items-center justify-center text-white font-extrabold text-[15px] shrink-0">
            {ballot.author.avatar_initials}
          </div>
          <div className="flex flex-col">
            <div className="flex items-center gap-1.5">
              <span className="font-['Plus_Jakarta_Sans'] text-[15px] font-bold text-white">
                {ballot.author.name}
              </span>
              {ballot.author.verified && (
                <span
                  className="material-symbols-outlined text-[#3B82F6] text-[16px]"
                  title="Verified Citizen Representative"
                >
                  verified
                </span>
              )}
            </div>
            <span className="text-[12px] text-[#64748B]">
              {ballot.author.role} • {ballot.closes_in ? `Closes ${ballot.closes_in}` : "Active"}
            </span>
          </div>
        </div>

        {/* Status Pill */}
        <span
          className={`inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-[11px] font-bold tracking-wide shrink-0 ${
            ballot.status === "passed"
              ? "bg-[#4F46E5]/20 text-[#c3c0ff] border border-[#4F46E5]/40"
              : ballot.status === "deliberation"
              ? "bg-[#3B82F6]/20 text-[#adc6ff] border border-[#3B82F6]/40"
              : "bg-[#10B981]/15 text-[#10B981] border border-[#10B981]/30"
          }`}
        >
          <span
            className={`w-2 h-2 rounded-full ${
              ballot.status === "passed"
                ? "bg-[#c3c0ff]"
                : ballot.status === "deliberation"
                ? "bg-[#3B82F6]"
                : "bg-[#10B981] animate-pulse"
            }`}
          ></span>
          {ballot.status_label || "Active Ballot"}
        </span>
      </div>

      {/* Question Title & Context */}
      <div className="flex flex-col gap-1.5">
        <div className="flex items-center gap-2 text-[#3B82F6] text-[11px] font-bold uppercase tracking-wider">
          <span>Proposal #{ballot.proposal_code}</span>
          <span>•</span>
          <span>{ballot.circle}</span>
        </div>
        <h2 className="font-['Plus_Jakarta_Sans'] text-[18px] sm:text-[20px] text-white font-bold tracking-tight leading-snug">
          {ballot.title}
        </h2>
        <p className="text-[14px] text-[#94A3B8] leading-relaxed mt-0.5">{ballot.summary}</p>
      </div>

      {/* Interactive Consensus Progress Bar & Breakdown */}
      <div className="bg-[#0A0E17] p-4 rounded-xl border border-[#222D42] flex flex-col gap-2.5">
        <div className="flex items-center justify-between text-[13px]">
          <span className="font-semibold text-white flex items-center gap-1.5">
            <span className="material-symbols-outlined text-[#10B981] text-[18px]">
              how_to_reg
            </span>
            Consensus Distribution
          </span>
          <span className="text-[#10B981] font-mono text-[12px] font-bold">
            Quorum Reached: {tally.quorum_pct}%
          </span>
        </div>

        {/* Segmented Color Track */}
        <div className="w-full h-3 rounded-full bg-[#262A34] overflow-hidden flex">
          <div
            className="bg-[#10B981] h-full transition-all duration-700 ease-out"
            style={{ width: `${tally.agree_pct}%` }}
            title={`Agree: ${tally.agree_pct}%`}
          />
          <div
            className="bg-[#EF4444] h-full transition-all duration-700 ease-out"
            style={{ width: `${tally.oppose_pct}%` }}
            title={`Oppose: ${tally.oppose_pct}%`}
          />
          <div
            className="bg-[#F59E0B] h-full transition-all duration-700 ease-out"
            style={{ width: `${tally.neutral_pct}%` }}
            title={`Neutral: ${tally.neutral_pct}%`}
          />
        </div>

        {/* Stats Breakdown Bar */}
        <div className="flex items-center justify-between flex-wrap gap-2 text-[12px] pt-1">
          <div className="flex items-center gap-3 font-bold">
            <span className="flex items-center gap-1 text-[#10B981]">
              <span className="w-2 h-2 rounded-full bg-[#10B981]"></span>
              {tally.agree_pct}% Agree
            </span>
            <span className="flex items-center gap-1 text-[#EF4444]">
              <span className="w-2 h-2 rounded-full bg-[#EF4444]"></span>
              {tally.oppose_pct}% Oppose
            </span>
            <span className="flex items-center gap-1 text-[#F59E0B]">
              <span className="w-2 h-2 rounded-full bg-[#F59E0B]"></span>
              {tally.neutral_pct}% Neutral
            </span>
          </div>
          <div className="text-[#64748B] text-[11px] font-mono">
            <strong className="text-white">{tally.total_ballots.toLocaleString()}</strong> verified
            ballots
          </div>
        </div>
      </div>

      {/* Action Footer Buttons */}
      <div className="flex items-center justify-between flex-wrap gap-3 pt-1 border-t border-[#222D42]/60">
        <div className="flex items-center gap-2">
          <button
            onClick={() => handleVoteClick("agree")}
            disabled={voting}
            className={`inline-flex items-center gap-1.5 px-4 py-2 rounded-full text-[13px] font-bold transition-all active:scale-95 ${
              votedChoice === "agree"
                ? "bg-[#10B981] text-white shadow-[0_0_12px_rgba(16,185,129,0.5)]"
                : "bg-[#10B981]/15 hover:bg-[#10B981]/25 text-[#10B981] border border-[#10B981]/30"
            }`}
          >
            <span className="material-symbols-outlined text-[16px]">thumb_up</span>
            <span>Vote Support</span>
          </button>

          <button
            onClick={() => handleVoteClick("oppose")}
            disabled={voting}
            className={`inline-flex items-center gap-1.5 px-4 py-2 rounded-full text-[13px] font-bold transition-all active:scale-95 ${
              votedChoice === "oppose"
                ? "bg-[#EF4444] text-white shadow-[0_0_12px_rgba(239,68,68,0.5)]"
                : "bg-[#EF4444]/15 hover:bg-[#EF4444]/25 text-[#EF4444] border border-[#EF4444]/30"
            }`}
          >
            <span className="material-symbols-outlined text-[16px]">thumb_down</span>
            <span>Vote Dissent</span>
          </button>

          <button
            onClick={() => handleVoteClick("neutral")}
            disabled={voting}
            className={`px-3 py-2 rounded-full text-[13px] font-semibold transition-all ${
              votedChoice === "neutral"
                ? "bg-[#F59E0B] text-black font-bold"
                : "bg-[#262A34] hover:bg-[#1C263A] text-[#94A3B8] hover:text-white"
            }`}
          >
            Abstain
          </button>
        </div>

        <div className="flex items-center gap-2">
          {ballot.deliberation_count > 0 && (
            <button
              onClick={() => onOpenDeliberation?.(ballot)}
              className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full bg-[#0A0E17] hover:bg-[#1C263A] text-[#c7c4d8] hover:text-white text-[12px] font-semibold border border-[#222D42] transition-colors"
            >
              <span className="material-symbols-outlined text-[16px]">forum</span>
              <span>Deliberation ({ballot.deliberation_count})</span>
            </button>
          )}

          <button
            onClick={() => onAuditHash(ballot)}
            className="p-2 rounded-full bg-[#0A0E17] hover:bg-[#1C263A] text-[#64748B] hover:text-white border border-[#222D42] transition-colors"
            title="Audit Hash on Precinct Ledger (Blake3 Proof)"
          >
            <span className="material-symbols-outlined text-[18px]">verified_user</span>
          </button>
        </div>
      </div>
    </article>
  );
}
