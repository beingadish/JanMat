import { pct } from "../lib/civic";
import type { Post } from "../store/store";
import { useCivic } from "../store/store";
import { Receipt } from "./ui";

/** The one Twitter-inspired piece: a clean poll that flips to result bars. */
export function Poll({ post }: { post: Post }) {
  const { vote, reveal } = useCivic();
  const poll = post.poll!;
  const total = Object.values(poll.tally).reduce((a, b) => a + b, 0);
  const showResults = Boolean(poll.myVote) || poll.revealed;

  return (
    <div className="poll">
      {!showResults
        ? poll.options.map((o) => (
            <button key={o} className="poll-choice" onClick={() => void vote(post.id, o)}>
              {o}
            </button>
          ))
        : poll.options.map((o) => {
            const c = poll.tally[o] ?? 0;
            const percent = pct(c, total);
            const mine = poll.myVote === o;
            return (
              <div key={o} className={`poll-result ${mine ? "mine" : ""}`}>
                <div className="poll-bar" style={{ width: `${percent}%` }} />
                <span className="poll-line">
                  <span className="poll-opt">
                    {o}
                    {mine && <span className="you"> · your vote</span>}
                  </span>
                  <span className="poll-pct mono">{percent}%</span>
                </span>
              </div>
            );
          })}

      {!showResults && (
        <button className="link poll-peek" onClick={() => void reveal(post.id)}>
          View results without voting
        </button>
      )}

      {poll.lastReceipt && <Receipt label="ZK vote receipt" data={poll.lastReceipt} />}
    </div>
  );
}
