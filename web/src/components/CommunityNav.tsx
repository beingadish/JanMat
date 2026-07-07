import { useState } from "react";
import { useCivic } from "../store/store";
import { Button, CommunityTile, Icon } from "./ui";

export function CommunityNav() {
  const { state, setTopic, setView, createTopic } = useCivic();
  const { topics, posts, topicFilter, view } = state;
  const [creating, setCreating] = useState(false);
  const [slug, setSlug] = useState("");
  const [busy, setBusy] = useState(false);

  const postCount = (topicId: string) =>
    posts.filter((p) => p.topicId === topicId).length;

  async function create() {
    if (!slug.trim()) return;
    setBusy(true);
    const t = await createTopic(slug);
    setBusy(false);
    if (t) {
      setSlug("");
      setCreating(false);
    }
  }

  return (
    <nav className="cnav">
      <button
        className={`cnav-item ${view === "feed" && !topicFilter ? "active" : ""}`}
        onClick={() => setTopic(null)}
      >
        <Icon name="home" /> <span>Home</span>
      </button>
      <button
        className={`cnav-item ${view === "governance" ? "active" : ""}`}
        onClick={() => setView("governance")}
      >
        <Icon name="gov" /> <span>Rules &amp; voting</span>
      </button>

      <div className="cnav-head">
        <span>Communities</span>
        <button
          className="link"
          onClick={() => setCreating((c) => !c)}
          title="Create community"
        >
          {creating ? "cancel" : "+ create"}
        </button>
      </div>

      {creating && (
        <div className="cnav-create">
          <div className="prefixed">
            <span className="prefix">j/</span>
            <input
              autoFocus
              value={slug}
              placeholder="community-name"
              onChange={(e) => setSlug(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && void create()}
            />
          </div>
          <Button loading={busy} disabled={!slug.trim()} onClick={create}>
            Create
          </Button>
        </div>
      )}

      <ul className="cnav-list">
        {topics.map((t) => (
          <li key={t.id}>
            <button
              className={`cnav-community ${topicFilter === t.id ? "active" : ""}`}
              onClick={() => setTopic(t.id)}
            >
              <CommunityTile slug={t.slug} size={26} />
              <span className="cname">j/{t.slug}</span>
              <span className="ccount">{postCount(t.id)}</span>
            </button>
          </li>
        ))}
      </ul>
    </nav>
  );
}
