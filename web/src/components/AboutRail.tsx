import { useEffect, useState } from "react";
import { api } from "../api/client";
import type { ConstitutionResponse } from "../api/types";
import { timeAgo } from "../lib/civic";
import { friendlyRule } from "../lib/constitution";
import { useCivic } from "../store/store";
import { CommunityTile, Icon } from "./ui";

export function AboutRail() {
  const { state, setSub } = useCivic();
  const { topics, posts, topicFilter, subFilter } = state;
  const community = topics.find((t) => t.id === topicFilter);

  const [constitution, setConstitution] = useState<ConstitutionResponse | null>(null);
  const [apiUp, setApiUp] = useState<boolean | null>(null);
  useEffect(() => {
    api.health().then((h) => setApiUp(h.status === "ok")).catch(() => setApiUp(false));
    api.constitution().then(setConstitution).catch(() => setConstitution(null));
  }, []);

  if (community) {
    const count = posts.filter((p) => p.topicId === community.id).length;
    return (
      <aside className="rail">
        <div className="card">
          <div className="about-head">
            <CommunityTile slug={community.slug} size={40} />
            <div>
              <div className="about-title">j/{community.slug}</div>
              <div className="muted small">created {timeAgo(community.createdAt)}</div>
            </div>
          </div>
          <div className="about-stats">
            <div>
              <b>{count}</b>
              <span className="muted small">opinions</span>
            </div>
            <div>
              <b>{community.subtopics.length}</b>
              <span className="muted small">subtopics</span>
            </div>
          </div>
          <div className="about-subs">
            <span className="rail-label">Subtopics</span>
            <div className="chips">
              <button
                className={`chip ${!subFilter ? "on" : ""}`}
                onClick={() => setSub(null)}
              >
                all
              </button>
              {community.subtopics.map((s) => (
                <button
                  key={s.id}
                  className={`chip ${subFilter === s.id ? "on" : ""}`}
                  onClick={() => setSub(s.id)}
                >
                  st/{s.slug}
                </button>
              ))}
            </div>
          </div>
        </div>
      </aside>
    );
  }

  return (
    <aside className="rail">
      <div className="card">
        <div className="rail-label">Platform</div>
        <div className="status-row">
          <span className={`dot ${apiUp === null ? "pending" : apiUp ? "up" : "down"}`} />
          janmat-api {apiUp === null ? "…" : apiUp ? "online" : "offline"}
        </div>
        <div className="about-stats">
          <div>
            <b>{topics.length}</b>
            <span className="muted small">communities</span>
          </div>
          <div>
            <b>{posts.length}</b>
            <span className="muted small">opinions</span>
          </div>
        </div>
      </div>

      <div className="card">
        <div className="rail-label">
          <Icon name="gov" size={14} /> How decisions work
        </div>
        {constitution ? (
          <ul className="rules-list">
            {constitution.rules.map((r) => {
              const f = friendlyRule(r.key, r.value);
              return (
                <li key={r.key} title={f.desc}>
                  <span className="rule-title">{f.title}</span>
                  <span className="rule-val mono">{f.display}</span>
                </li>
              );
            })}
          </ul>
        ) : (
          <p className="muted small">unavailable</p>
        )}
      </div>
    </aside>
  );
}
