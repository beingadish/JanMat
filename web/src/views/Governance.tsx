import { useEffect, useState } from "react";
import { api } from "../api/client";
import type { ConstitutionResponse, CreateProposalResponse } from "../api/types";
import { Button, Receipt } from "../components/ui";
import { timeAgo } from "../lib/civic";
import { friendlyRule } from "../lib/constitution";
import { useCivic } from "../store/store";

interface LocalProposal extends CreateProposalResponse {
  title: string;
  type: string;
  createdAt: number;
}

// Plain-language labels for the two change types (API still uses regular/constitutional).
const CHANGE_TYPES = [
  { value: "regular", label: "Everyday change — passes with a simple majority" },
  { value: "constitutional", label: "Rule change — needs a large majority (supermajority)" },
] as const;

export function Governance() {
  const { state } = useCivic();
  const [rulebook, setRulebook] = useState<ConstitutionResponse | null>(null);
  const [type, setType] = useState<"regular" | "constitutional">("regular");
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [busy, setBusy] = useState(false);
  const [proposals, setProposals] = useState<LocalProposal[]>([]);
  const [err, setErr] = useState<string | null>(null);

  useEffect(() => {
    api.constitution().then(setRulebook).catch(() => setRulebook(null));
  }, []);

  async function suggest() {
    if (!title.trim()) return;
    setBusy(true);
    setErr(null);
    try {
      const r = await api.createProposal({
        proposal_type: type,
        title: title.trim(),
        description: description.trim(),
        proposer_wallet: state.identity?.nullifier ?? "anonymous",
      });
      setProposals((ps) => [{ ...r, title: title.trim(), type, createdAt: Date.now() }, ...ps]);
      setTitle("");
      setDescription("");
    } catch (e) {
      setErr((e as Error).message);
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="gov-view">
      <h1>How JanMat works</h1>
      <p className="muted" style={{ marginTop: 0 }}>
        The plain-language rules every community runs on — and where you can
        suggest a change.
      </p>

      <div className="card">
        <div className="rail-label">The rulebook</div>
        {rulebook ? (
          <ul className="rule-cards">
            {rulebook.rules.map((r) => {
              const f = friendlyRule(r.key, r.value);
              return (
                <li key={r.key} className="rule-card">
                  <div className="rule-card-main">
                    <span className="rule-card-title">{f.title}</span>
                    <span className="rule-card-desc muted small">{f.desc}</span>
                  </div>
                  <span className="rule-card-val mono">{f.display}</span>
                </li>
              );
            })}
          </ul>
        ) : (
          <p className="muted small">unavailable</p>
        )}
      </div>

      <div className="card">
        <div className="rail-label">Suggest a change</div>
        <p className="muted small" style={{ marginTop: 0 }}>
          Propose something for your community to vote on.
        </p>
        <div className="prop-form">
          <label className="field">
            <span>Type of change</span>
            <select value={type} onChange={(e) => setType(e.target.value as "regular" | "constitutional")}>
              {CHANGE_TYPES.map((c) => (
                <option key={c.value} value={c.value}>
                  {c.label}
                </option>
              ))}
            </select>
          </label>
          <label className="field">
            <span>What should change?</span>
            <input placeholder="e.g. Add a covered bus shelter at the market stop" value={title} onChange={(e) => setTitle(e.target.value)} />
          </label>
          <label className="field">
            <span>Why? (optional)</span>
            <textarea placeholder="Explain the reason so others can decide…" rows={3} value={description} onChange={(e) => setDescription(e.target.value)} />
          </label>
          {err && <div className="inline-err">{err}</div>}
          <Button loading={busy} disabled={!title.trim()} onClick={suggest}>
            Suggest it
          </Button>
        </div>
      </div>

      {proposals.length > 0 && (
        <div className="card">
          <div className="rail-label">Suggestions you've made</div>
          <ul className="prop-list">
            {proposals.map((p) => (
              <li key={p.proposal_id} className="prop-item">
                <div className="prop-top">
                  <span className="prop-title">{p.title}</span>
                  <span className={`tag ${p.type}`}>
                    {p.type === "constitutional" ? "rule change" : "everyday"}
                  </span>
                </div>
                <div className="muted small">
                  needs {Math.round(p.required_quorum * 100)}% in favour · voting closes in ~
                  {p.closes_at_block} blocks · {timeAgo(p.createdAt)}
                </div>
                <Receipt label="details" data={p} />
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}
