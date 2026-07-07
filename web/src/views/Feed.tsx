import { PostComposer } from "../components/PostComposer";
import { PostCard } from "../components/PostCard";
import { CommunityTile } from "../components/ui";
import { useCivic } from "../store/store";

export function Feed() {
  const { state, setSub } = useCivic();
  const { posts, topics, topicFilter, subFilter } = state;
  const community = topics.find((t) => t.id === topicFilter);

  const visible = posts.filter(
    (p) =>
      (!topicFilter || p.topicId === topicFilter) &&
      (!subFilter || p.subtopicId === subFilter),
  );

  return (
    <div className="feed">
      {community ? (
        <div className="community-banner card">
          <div className="cb-head">
            <CommunityTile slug={community.slug} size={44} />
            <div>
              <h1 className="cb-title">j/{community.slug}</h1>
              <p className="muted small">
                {posts.filter((p) => p.topicId === community.id).length} opinions ·{" "}
                {community.subtopics.length} subtopics
              </p>
            </div>
          </div>
          <div className="cb-tabs">
            <button className={`tab ${!subFilter ? "on" : ""}`} onClick={() => setSub(null)}>
              All
            </button>
            {community.subtopics.map((s) => (
              <button
                key={s.id}
                className={`tab ${subFilter === s.id ? "on" : ""}`}
                onClick={() => setSub(s.id)}
              >
                st/{s.slug}
              </button>
            ))}
          </div>
        </div>
      ) : (
        <div className="home-head">
          <h1>Home</h1>
          <p className="muted">Latest opinions across every community.</p>
        </div>
      )}

      <PostComposer />

      {visible.length === 0 ? (
        <div className="empty card">
          {topicFilter
            ? "No opinions here yet — be the first to post."
            : "No opinions yet. Pick a community and share the first opinion."}
        </div>
      ) : (
        visible.map((p) => <PostCard key={p.id} post={p} />)
      )}
    </div>
  );
}
