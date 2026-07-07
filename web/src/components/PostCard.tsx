import { shortId, timeAgo } from "../lib/civic";
import type { Post } from "../store/store";
import { useCivic } from "../store/store";
import { CommunityTile, Receipt } from "./ui";
import { Poll } from "./Poll";

export function PostCard({ post }: { post: Post }) {
  const { setTopic, setSub } = useCivic();
  const total = post.poll
    ? Object.values(post.poll.tally).reduce((a, b) => a + b, 0)
    : 0;

  return (
    <article className="post card">
      <div className="post-meta">
        <CommunityTile slug={post.topicSlug} size={22} />
        <button className="link-strong" onClick={() => setTopic(post.topicId)}>
          j/{post.topicSlug}
        </button>
        <button
          className="sub-pill"
          onClick={() => {
            setTopic(post.topicId);
            setSub(post.subtopicId);
          }}
        >
          st/{post.subtopicSlug}
        </button>
        <span className="post-byline muted">
          @{post.authorHandle} · {timeAgo(post.createdAt)}
        </span>
      </div>

      <p className="post-body">{post.content}</p>

      {post.poll && <Poll post={post} />}

      <div className="post-foot">
        {post.poll && (
          <span className="muted small">
            {total.toLocaleString()} vote{total === 1 ? "" : "s"}
          </span>
        )}
        <span className="muted small mono">{shortId(post.id)}</span>
        {(post.cid || post.contentHash) && (
          <Receipt
            label="receipt"
            data={{
              opinion_id: post.id,
              subtopic_id: post.subtopicId,
              content_cid: post.cid,
              content_hash: post.contentHash,
              verdict: post.verdict,
            }}
          />
        )}
      </div>
    </article>
  );
}
