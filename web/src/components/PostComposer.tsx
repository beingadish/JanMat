import { useEffect, useMemo, useState } from "react";
import { useCivic } from "../store/store";
import { Avatar, Button, Icon } from "./ui";

const MAX = 500;

export function PostComposer() {
  const { state, publishPost, addSubtopic, setView } = useCivic();
  const { identity, topics, topicFilter } = state;

  const [communityId, setCommunityId] = useState(topicFilter ?? topics[0]?.id ?? "");
  const [subId, setSubId] = useState("");
  const [content, setContent] = useState("");
  const [withPoll, setWithPoll] = useState(false);
  const [options, setOptions] = useState<string[]>(["Yes", "No"]);
  const [newSub, setNewSub] = useState("");
  const [addingSub, setAddingSub] = useState(false);
  const [busy, setBusy] = useState(false);

  // Follow the community the user is browsing.
  useEffect(() => {
    if (topicFilter) setCommunityId(topicFilter);
  }, [topicFilter]);

  const community = useMemo(
    () => topics.find((t) => t.id === communityId),
    [topics, communityId],
  );

  // Keep the selected subtopic valid for the current community.
  useEffect(() => {
    if (community && !community.subtopics.some((s) => s.id === subId)) {
      setSubId(community.subtopics[0]?.id ?? "");
    }
  }, [community, subId]);

  if (!identity) {
    return (
      <div className="card composer-cta">
        <p>Register to open communities, post opinions and vote.</p>
        <Button onClick={() => setView("identity")}>
          <Icon name="id" size={16} /> Register / Log in
        </Button>
      </div>
    );
  }

  const setOpt = (i: number, v: string) =>
    setOptions((o) => o.map((x, j) => (j === i ? v : x)));
  const canPost = !!content.trim() && !!communityId && !!subId;
  const over = content.length > MAX * 0.9;

  function commitSub() {
    if (!newSub.trim() || !community) return;
    const s = addSubtopic(community.id, newSub);
    if (s) {
      setSubId(s.id);
      setNewSub("");
      setAddingSub(false);
    }
  }

  async function post() {
    if (!canPost) return;
    setBusy(true);
    const ok = await publishPost({
      topicId: communityId,
      subtopicId: subId,
      content: content.trim(),
      pollOptions: withPoll ? options : undefined,
    });
    setBusy(false);
    if (ok) {
      setContent("");
      setWithPoll(false);
      setOptions(["Yes", "No"]);
    }
  }

  return (
    <div className="card composer">
      <div className="composer-top">
        <Avatar seed={identity.handle} size={36} />
        <textarea
          className="composer-text"
          placeholder="Share a civic opinion…"
          maxLength={MAX}
          rows={2}
          value={content}
          onChange={(e) => setContent(e.target.value)}
        />
      </div>

      {withPoll && (
        <div className="poll-editor">
          <span className="editor-label">Poll choices</span>
          {options.map((o, i) => (
            <div key={i} className="poll-editor-row">
              <input
                value={o}
                placeholder={`Choice ${i + 1}`}
                maxLength={40}
                onChange={(e) => setOpt(i, e.target.value)}
              />
              {options.length > 2 && (
                <button className="link" onClick={() => setOptions((o2) => o2.filter((_, j) => j !== i))}>
                  remove
                </button>
              )}
            </div>
          ))}
          {options.length < 4 && (
            <button className="link" onClick={() => setOptions((o) => [...o, ""])}>
              + add choice
            </button>
          )}
        </div>
      )}

      <div className="composer-bar">
        <div className="composer-selects">
          <label className="sel">
            <span className="sel-prefix">j/</span>
            <select value={communityId} onChange={(e) => setCommunityId(e.target.value)}>
              {topics.map((t) => (
                <option key={t.id} value={t.id}>
                  {t.slug}
                </option>
              ))}
            </select>
          </label>

          {!addingSub ? (
            <label className="sel">
              <span className="sel-prefix">st/</span>
              <select value={subId} onChange={(e) => setSubId(e.target.value)}>
                {community?.subtopics.map((s) => (
                  <option key={s.id} value={s.id}>
                    {s.slug}
                  </option>
                ))}
              </select>
              <button className="sel-add" title="New subtopic" onClick={() => setAddingSub(true)}>
                <Icon name="plus" size={14} />
              </button>
            </label>
          ) : (
            <div className="prefixed">
              <span className="prefix">st/</span>
              <input
                autoFocus
                value={newSub}
                placeholder="new-subtopic"
                onChange={(e) => setNewSub(e.target.value)}
                onKeyDown={(e) => e.key === "Enter" && commitSub()}
              />
              <button className="link" onClick={commitSub}>add</button>
            </div>
          )}

          <button
            className={`iconbtn ${withPoll ? "on" : ""}`}
            title="Attach a poll"
            onClick={() => setWithPoll((p) => !p)}
          >
            <Icon name="poll" size={16} /> <span>Poll</span>
          </button>
        </div>

        <div className="composer-post">
          <span className={`counter ${over ? "warn-text" : "muted"}`}>{MAX - content.length}</span>
          <Button loading={busy} disabled={!canPost} onClick={post}>
            Post
          </Button>
        </div>
      </div>
    </div>
  );
}
