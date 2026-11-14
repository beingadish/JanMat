import { useState } from "react";
import type { BallotItem, CommentItem } from "../api/types";
import { formatRelativeTime, useNow } from "../lib/time";

interface BallotCardProps {
  ballot: BallotItem;
  isLoggedIn?: boolean;
  onRequireLogin?: () => void;
  userVotedChoice?: "agree" | "oppose" | "neutral" | null;
  onVote: (ballotId: string, choice: "agree" | "oppose" | "neutral") => Promise<void>;
  onAuditHash: (ballot: BallotItem) => void;
  onAddComment?: (ballotId: string, content: string, authorName?: string) => Promise<void>;
  onReshareComment?: (commentId: string, question: string, authorName?: string) => Promise<void>;
  onUpvoteComment?: (commentId: string) => Promise<void>;
  isCommentEndorsed?: (commentId: string) => boolean;
  onOpenDeliberation?: (ballot: BallotItem) => void;
}

export function BallotCard({
  ballot,
  isLoggedIn = false,
  onRequireLogin,
  userVotedChoice,
  onVote,
  onAuditHash,
  onAddComment,
  onReshareComment,
  onUpvoteComment,
  isCommentEndorsed,
}: BallotCardProps) {
  const [voting, setVoting] = useState(false);
  const [votedChoice, setVotedChoice] = useState<string | null>(null);
  const now = useNow(5000); // 5s local reactive timer for relative timestamps

  const effectiveChoice = userVotedChoice || votedChoice;
  const hasVoted = !!effectiveChoice;

  // Deliberation Tray state
  const [showDeliberation, setShowDeliberation] = useState(ballot.is_official || false);
  const [newComment, setNewComment] = useState("");
  const [submittingComment, setSubmittingComment] = useState(false);

  // Re-share / Spinoff question state: commentId -> active prompt
  const [reshareTargetId, setReshareTargetId] = useState<string | null>(null);
  const [reshareQuestion, setReshareQuestion] = useState("");
  const [submittingReshare, setSubmittingReshare] = useState(false);

  const handleVoteClick = async (choice: "agree" | "oppose" | "neutral") => {
    if (!isLoggedIn) {
      onRequireLogin?.();
      return;
    }
    // Under JanMat Constitution: votes are strictly immutable. Once cast, changing votes is impossible.
    if (hasVoted || voting) return;
    setVoting(true);
    try {
      await onVote(ballot.id, choice);
      setVotedChoice(choice);
    } finally {
      setVoting(false);
    }
  };

  const handlePostComment = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!isLoggedIn) {
      onRequireLogin?.();
      return;
    }
    if (!newComment.trim() || submittingComment) return;
    setSubmittingComment(true);
    try {
      if (onAddComment) {
        await onAddComment(ballot.id, newComment.trim());
        setNewComment("");
      }
    } finally {
      setSubmittingComment(false);
    }
  };

  const handleReshareSubmit = async (commentId: string) => {
    if (!isLoggedIn) {
      onRequireLogin?.();
      return;
    }
    if (!reshareQuestion.trim() || submittingReshare) return;
    setSubmittingReshare(true);
    try {
      if (onReshareComment) {
        await onReshareComment(commentId, reshareQuestion.trim());
        setReshareTargetId(null);
        setReshareQuestion("");
      }
    } finally {
      setSubmittingReshare(false);
    }
  };

  const commentsList: CommentItem[] = ballot.comments || [];
  const commentsCount = commentsList.length || ballot.deliberation_count || 0;
  const { tally } = ballot;

  return (
    <article
      className={`bg-[#131A29] rounded-2xl p-5 sm:p-6 border shadow-md flex flex-col gap-4 transition-all hover:shadow-xl ${
        ballot.is_official
          ? "border-[#4F46E5]/60 hover:border-[#6366F1] shadow-[0_0_20px_rgba(79,70,229,0.12)]"
          : "border-[#222D42] hover:border-[#3E4C6D]"
      }`}
    >
      {/* Official Foundation Banner if applicable */}
      {ballot.is_official && (
        <div className="flex items-center justify-between px-3 py-1.5 rounded-lg bg-gradient-to-r from-[#4F46E5]/20 via-[#3B82F6]/15 to-transparent border border-[#4F46E5]/40 text-[#c3c0ff] text-[12px] font-semibold">
          <div className="flex items-center gap-2">
            <span className="material-symbols-outlined text-[16px] text-[#818CF8]">account_balance</span>
            <span className="font-bold tracking-wide">Official JanMat Foundation Poll</span>
          </div>
          <span className="text-[11px] text-[#94A3B8] font-mono">Consensus-Tracked</span>
        </div>
      )}

      {/* Card Header Meta */}
      <div className="flex items-start justify-between gap-3 flex-wrap sm:flex-nowrap">
        <div className="flex items-center gap-3">
          <div
            className={`w-10 h-10 rounded-full flex items-center justify-center text-white font-extrabold text-[15px] shrink-0 shadow-sm ${
              ballot.is_official ? "bg-[#4F46E5]" : "bg-[#0566D9]"
            }`}
          >
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
                  title="Verified Representative"
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
        {(() => {
          const rawLabel = ballot.status_label || (ballot.closes_in ? `Closes in ${ballot.closes_in}` : "Active");
          const cleanLabel = rawLabel
            .replace(/^Spun-off Community Motion\s*[•·-]\s*/i, "")
            .replace(/^Spun-off Community Motion/i, "Active")
            .trim();
          return (
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
              {cleanLabel}
            </span>
          );
        })()}
      </div>

      {/* Quoted Comment Context (if re-shared as a spinoff poll question) */}
      {ballot.quoted_comment && (
        <div className="bg-[#0A0E17]/60 rounded-xl p-3 border-l-2 border-[#3B82F6] border-y border-r border-[#222D42]/60 text-[13px] flex flex-col gap-1">
          {ballot.quoted_comment.original_author && (
            <div className="flex items-center justify-end text-[11px] text-[#64748B] font-mono lowercase">
              {ballot.quoted_comment.original_author}
            </div>
          )}
          <p className="text-[#DFE2EF] italic leading-relaxed text-[13.5px]">
            &ldquo;{ballot.quoted_comment.original_content}&rdquo;
          </p>
        </div>
      )}

      {/* Question Title & Deliberation Thesis */}
      <div className="flex flex-col gap-1">
        <h2 className="font-['Plus_Jakarta_Sans'] text-[18px] sm:text-[20px] text-white font-bold tracking-tight leading-snug">
          {ballot.title}
        </h2>
        {ballot.summary && !ballot.quoted_comment && !ballot.summary.startsWith("Spun-off") && !ballot.summary.includes("Re-posted suggestion") && (
          <p className="text-[14px] text-[#94A3B8] leading-relaxed mt-0.5">{ballot.summary}</p>
        )}
      </div>

      {/* Sleek Minimal Consensus Progress Bar & Breakdown */}
      <div className="flex flex-col gap-2 pt-1 pb-1">
        {/* Sleek Segmented Color Track */}
        <div className="w-full h-1.5 rounded-full bg-[#1C263A] overflow-hidden flex">
          <div
            className="bg-[#10B981] h-full transition-all duration-700 ease-out"
            style={{ width: `${tally.agree_pct}%` }}
          />
          <div
            className="bg-[#EF4444] h-full transition-all duration-700 ease-out"
            style={{ width: `${tally.oppose_pct}%` }}
          />
          <div
            className="bg-[#F59E0B] h-full transition-all duration-700 ease-out"
            style={{ width: `${tally.neutral_pct}%` }}
          />
        </div>

        {/* Sleek Stats Breakdown */}
        <div className="flex items-center justify-between text-[11px] text-[#94A3B8]">
          <div className="flex items-center gap-3">
            <span className="flex items-center gap-1.5">
              <span className="w-1.5 h-1.5 rounded-full bg-[#10B981]"></span>
              <span className="font-semibold text-white">{tally.agree_pct}%</span> Support
            </span>
            <span className="flex items-center gap-1.5">
              <span className="w-1.5 h-1.5 rounded-full bg-[#EF4444]"></span>
              <span className="font-semibold text-white">{tally.oppose_pct}%</span> Dissent
            </span>
            {tally.neutral_pct > 0 && (
              <span className="flex items-center gap-1.5">
                <span className="w-1.5 h-1.5 rounded-full bg-[#F59E0B]"></span>
                <span className="font-semibold text-white">{tally.neutral_pct}%</span> Neutral
              </span>
            )}
          </div>
          <span className="text-[#64748B] font-mono text-[11px]">
            {tally.total_ballots} {tally.total_ballots === 1 ? "vote" : "votes"}
          </span>
        </div>
      </div>

      {/* Action Footer Buttons */}
      <div className="flex items-center justify-between flex-wrap gap-3 pt-1 border-t border-[#222D42]/60">
        <div className="flex items-center gap-2">
          {/* Support Button */}
          <button
            onClick={() => handleVoteClick("agree")}
            disabled={hasVoted || voting}
            title={
              hasVoted
                ? "Ballot already anchored on-chain. Under the JanMat Constitution, votes cannot be changed."
                : !isLoggedIn
                ? "Citizen authentication required to vote"
                : "Support this motion"
            }
            className={`inline-flex items-center gap-1.5 px-4 py-2 rounded-full text-[13px] font-bold transition-all ${
              effectiveChoice === "agree"
                ? "bg-[#10B981] text-white shadow-[0_0_12px_rgba(16,185,129,0.5)] cursor-default ring-2 ring-[#10B981]/50"
                : hasVoted
                ? "opacity-30 cursor-not-allowed bg-[#10B981]/10 text-[#10B981] border border-[#10B981]/20"
                : "bg-[#10B981]/15 hover:bg-[#10B981]/25 text-[#10B981] border border-[#10B981]/30 active:scale-95"
            }`}
          >
            <span className="material-symbols-outlined text-[16px]">
              {effectiveChoice === "agree" ? "check_circle" : "thumb_up"}
            </span>
            <span>{effectiveChoice === "agree" ? "Supported" : "Support"}</span>
          </button>

          {/* Dissent Button */}
          <button
            onClick={() => handleVoteClick("oppose")}
            disabled={hasVoted || voting}
            title={
              hasVoted
                ? "Ballot already anchored on-chain. Under the JanMat Constitution, votes cannot be changed."
                : !isLoggedIn
                ? "Citizen authentication required to vote"
                : "Dissent against this motion"
            }
            className={`inline-flex items-center gap-1.5 px-4 py-2 rounded-full text-[13px] font-bold transition-all ${
              effectiveChoice === "oppose"
                ? "bg-[#EF4444] text-white shadow-[0_0_12px_rgba(239,68,68,0.5)] cursor-default ring-2 ring-[#EF4444]/50"
                : hasVoted
                ? "opacity-30 cursor-not-allowed bg-[#EF4444]/10 text-[#EF4444] border border-[#EF4444]/20"
                : "bg-[#EF4444]/15 hover:bg-[#EF4444]/25 text-[#EF4444] border border-[#EF4444]/30 active:scale-95"
            }`}
          >
            <span className="material-symbols-outlined text-[16px]">
              {effectiveChoice === "oppose" ? "cancel" : "thumb_down"}
            </span>
            <span>{effectiveChoice === "oppose" ? "Dissented" : "Dissent"}</span>
          </button>

          {/* Neutral Button */}
          <button
            onClick={() => handleVoteClick("neutral")}
            disabled={hasVoted || voting}
            title={
              hasVoted
                ? "Ballot already anchored on-chain. Under the JanMat Constitution, votes cannot be changed."
                : !isLoggedIn
                ? "Citizen authentication required to vote"
                : "Vote Neutral"
            }
            className={`inline-flex items-center gap-1.5 px-3.5 py-2 rounded-full text-[13px] font-semibold transition-all ${
              effectiveChoice === "neutral"
                ? "bg-[#F59E0B] text-black font-bold cursor-default ring-2 ring-[#F59E0B]/50"
                : hasVoted
                ? "opacity-30 cursor-not-allowed bg-[#262A34] text-[#94A3B8]"
                : "bg-[#262A34] hover:bg-[#1C263A] text-[#94A3B8] hover:text-white border border-[#222D42] active:scale-95"
            }`}
          >
            <span className="material-symbols-outlined text-[16px]">
              {effectiveChoice === "neutral" ? "check" : "balance"}
            </span>
            <span>{effectiveChoice === "neutral" ? "Neutral (Cast)" : "Neutral"}</span>
          </button>

          {hasVoted && (
            <span className="hidden sm:inline-flex items-center gap-1 px-2.5 py-1 rounded-full bg-[#10B981]/10 border border-[#10B981]/30 text-[11px] font-mono font-semibold text-[#10B981]">
              <span className="material-symbols-outlined text-[13px]">lock</span>
              Immutable Vote Anchored
            </span>
          )}
        </div>

        <div className="flex items-center gap-2">
          {/* Suggestions & Deliberations Toggle Button */}
          <button
            onClick={() => setShowDeliberation(!showDeliberation)}
            className={`inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-[12px] font-semibold border transition-all ${
              showDeliberation
                ? "bg-[#4F46E5] text-white border-[#4F46E5] shadow-sm"
                : "bg-[#0A0E17] hover:bg-[#1C263A] text-[#c7c4d8] hover:text-white border-[#222D42]"
            }`}
          >
            <span className="material-symbols-outlined text-[16px]">forum</span>
            <span>Suggestions ({commentsCount})</span>
            <span className="material-symbols-outlined text-[14px]">
              {showDeliberation ? "expand_less" : "expand_more"}
            </span>
          </button>

          {/* Cryptographic Proof Audit */}
          <button
            onClick={() => onAuditHash(ballot)}
            className="p-2 rounded-full bg-[#0A0E17] hover:bg-[#1C263A] text-[#64748B] hover:text-white border border-[#222D42] transition-colors"
            title="Audit Cryptographic Blake3 Receipt"
          >
            <span className="material-symbols-outlined text-[18px]">verified_user</span>
          </button>
        </div>
      </div>

      {/* Expandable Suggestions & Deliberations Tray */}
      {showDeliberation && (
        <section className="bg-[#0A0E17] rounded-xl p-4 border border-[#222D42] flex flex-col gap-4 mt-1 animate-fadeIn">
          <div className="flex items-center justify-between pb-2 border-b border-[#222D42]/60">
            <div className="flex items-center gap-2">
              <span className="material-symbols-outlined text-[#3B82F6] text-[18px]">chat_bubble</span>
              <h3 className="text-[13px] font-bold text-white uppercase tracking-wider">
                Community Suggestions & Deliberations
              </h3>
            </div>
            <span className="text-[11px] text-[#64748B] font-mono">
              Blake3 cryptographic ledger integrity
            </span>
          </div>

          {/* Suggestion Submission Form: Clean, native form without redundant login buttons */}
          <form onSubmit={handlePostComment} className="flex flex-col gap-2.5">
            <textarea
              value={newComment}
              onFocus={() => {
                if (!isLoggedIn) onRequireLogin?.();
              }}
              onChange={(e) => {
                if (!isLoggedIn) {
                  onRequireLogin?.();
                  return;
                }
                setNewComment(e.target.value);
              }}
              placeholder="Post a constructive suggestion or deliberation on this motion..."
              rows={2}
              className="w-full bg-[#131A29] text-[13px] text-white placeholder-[#64748B] px-3.5 py-2.5 rounded-xl border border-[#222D42] focus:outline-none focus:border-[#4F46E5] resize-none"
            />
            <div className="flex items-center justify-end gap-3">
              <button
                type="submit"
                disabled={submittingComment || (isLoggedIn && !newComment.trim())}
                className="inline-flex items-center gap-1.5 px-4 py-1.5 rounded-full bg-[#4F46E5] hover:bg-[#4338CA] text-white text-[12px] font-bold transition-all disabled:opacity-50 active:scale-95 shadow-sm"
              >
                <span className="material-symbols-outlined text-[15px]">send</span>
                <span>{submittingComment ? "Posting..." : "Post Suggestion"}</span>
              </button>
            </div>
          </form>

          {/* List of Comments & Spinoff Re-share Prompts */}
          <div className="flex flex-col gap-3 mt-1">
            {commentsList.length === 0 ? (
              <p className="text-[12px] text-[#64748B] text-center py-2 italic">
                No citizen suggestions yet. Be the first to add your perspective!
              </p>
            ) : (
              commentsList.map((c) => (
                <div
                  key={c.id}
                  className="bg-[#131A29] rounded-xl p-3.5 border border-[#222D42]/80 flex flex-col gap-2 transition-all hover:border-[#3E4C6D]"
                >
                  <div className="flex items-center justify-between gap-2 flex-wrap text-[12px]">
                    <div className="flex items-center gap-2">
                      <span className="font-bold text-white text-[13px]">{c.author_name}</span>
                      <span className="text-[#64748B] font-mono text-[11px]">@{c.author_handle}</span>
                      {c.created_at && (
                        <span className="text-[#64748B] text-[11px] font-mono">
                          • {formatRelativeTime(c.created_at, now)}
                        </span>
                      )}
                    </div>
                    <span
                      className="font-mono text-[10px] text-[#10B981] bg-[#10B981]/10 px-2 py-0.5 rounded border border-[#10B981]/20"
                      title={`Blake3 State Proof: ${c.blake3_hash}`}
                    >
                      {c.blake3_hash ? `Blake3:${c.blake3_hash.slice(0, 8)}...` : "Audited"}
                    </span>
                  </div>

                  <p className="text-[13px] text-[#D1D5DB] leading-relaxed">{c.content}</p>

                  {/* Comment Actions: Upvote & Re-share as Question */}
                  <div className="flex items-center justify-between pt-1 border-t border-[#222D42]/50 text-[12px]">
                    <div className="flex items-center gap-3">
                      <button
                        type="button"
                        onClick={() => {
                          if (!isLoggedIn) {
                            onRequireLogin?.();
                            return;
                          }
                          onUpvoteComment?.(c.id);
                        }}
                        className={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-[12px] font-medium transition-all ${
                          isCommentEndorsed?.(c.id)
                            ? "bg-[#10B981]/20 text-[#10B981] border border-[#10B981]/40 font-bold"
                            : "text-[#94A3B8] hover:text-[#10B981] hover:bg-[#1C263A] border border-transparent"
                        }`}
                        title={
                          !isLoggedIn
                            ? "Log in to endorse"
                            : isCommentEndorsed?.(c.id)
                            ? "Click to revoke endorsement"
                            : "Endorse this suggestion (1 Citizen = 1 Vote)"
                        }
                      >
                        <span className="material-symbols-outlined text-[15px]">
                          {isCommentEndorsed?.(c.id) ? "thumb_up" : "thumb_up"}
                        </span>
                        <span>{c.upvotes}</span>
                        {isCommentEndorsed?.(c.id) && (
                          <span className="text-[10px] uppercase font-bold text-[#10B981]">Endorsed</span>
                        )}
                      </button>

                      {c.reshare_count > 0 && (
                        <span className="text-[11px] text-[#64748B] font-mono">
                          Spun off {c.reshare_count}x
                        </span>
                      )}
                    </div>

                    <button
                      onClick={() => {
                        if (!isLoggedIn) {
                          onRequireLogin?.();
                          return;
                        }
                        if (reshareTargetId === c.id) {
                          setReshareTargetId(null);
                        } else {
                          setReshareTargetId(c.id);
                          setReshareQuestion(`Do you like this idea?`);
                        }
                      }}
                      className="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-[#1C263A] hover:bg-[#222D42] text-[#3B82F6] hover:text-[#60A5FA] text-[11px] font-semibold border border-[#3B82F6]/30 transition-all"
                    >
                      <span className="material-symbols-outlined text-[14px]">repeat</span>
                      <span>Re-share with Question</span>
                    </button>
                  </div>

                  {/* Inline Re-share Spinoff Poll Prompt */}
                  {reshareTargetId === c.id && (
                    <div className="bg-[#0A0E17] rounded-lg p-3 border border-[#3B82F6]/40 flex flex-col gap-2 mt-1 animate-fadeIn">
                      <div className="flex items-center justify-between text-[11px] text-[#93C5FD]">
                        <span className="font-bold flex items-center gap-1">
                          <span className="material-symbols-outlined text-[14px]">tune</span>
                          Ask a Community Question for this Suggestion:
                        </span>
                        <button
                          onClick={() => setReshareTargetId(null)}
                          className="text-[#64748B] hover:text-white"
                        >
                          ✕
                        </button>
                      </div>
                      <input
                        type="text"
                        value={reshareQuestion}
                        onChange={(e) => setReshareQuestion(e.target.value)}
                        placeholder="e.g. Do you like this idea? Should JanMat implement this?"
                        className="bg-[#131A29] text-[12px] text-white px-3 py-1.5 rounded-lg border border-[#222D42] focus:outline-none focus:border-[#3B82F6]"
                      />
                      <div className="flex items-center justify-end gap-2">
                        <button
                          type="button"
                          onClick={() => setReshareTargetId(null)}
                          className="px-3 py-1 rounded-full text-[11px] text-[#94A3B8] hover:text-white"
                        >
                          Cancel
                        </button>
                        <button
                          type="button"
                          disabled={submittingReshare || !reshareQuestion.trim()}
                          onClick={() => handleReshareSubmit(c.id)}
                          className="px-3.5 py-1 rounded-full bg-[#3B82F6] hover:bg-[#2563EB] text-white text-[11px] font-bold transition-all disabled:opacity-50"
                        >
                          {submittingReshare ? "Creating..." : "Launch Spinoff Poll"}
                        </button>
                      </div>
                    </div>
                  )}
                </div>
              ))
            )}
          </div>
        </section>
      )}
    </article>
  );
}
