import type { ZkReceipt } from "../api/types";

/** Namespace ids look like "v/01H…" / "t/01H…"; path params take just the ULID. */
export function pathSeg(id: string): string {
  return id.includes("/") ? id.slice(id.indexOf("/") + 1) : id;
}

/** Expand a string into a deterministic 32-byte array (0–255). */
function hash32(input: string): number[] {
  const out: number[] = [];
  let i = 0;
  while (out.length < 32) {
    let h = 0x811c9dc5 ^ i;
    const s = `${input}:${i}`;
    for (let k = 0; k < s.length; k++) {
      h ^= s.charCodeAt(k);
      h = Math.imul(h, 0x01000193);
    }
    out.push(h & 0xff, (h >>> 8) & 0xff, (h >>> 16) & 0xff, (h >>> 24) & 0xff);
    i++;
  }
  return out.slice(0, 32);
}

/**
 * Build a Zk vote receipt the backend accepts. `public_outputs` is the UTF-8
 * JSON of a VoteCastOutput { nullifier[32], commitment[32], vote_id } whose
 * vote_id must equal the path segment (see pallet-voting cast_vote).
 */
export function buildVoteReceipt(
  voteKey: string,
  seed: string,
  option: string,
): ZkReceipt {
  const output = {
    nullifier: hash32(`null:${seed}:${voteKey}`),
    commitment: hash32(`comm:${option}:${voteKey}:${Math.random()}`),
    vote_id: voteKey,
  };
  const public_outputs = Array.from(
    new TextEncoder().encode(JSON.stringify(output)),
  );
  return {
    circuit_id: "janmat/vote-cast/v1",
    proof_bytes: [1, 2, 3, 4],
    public_outputs,
  };
}

/** Deterministic pseudonymous handle derived from a nullifier/commitment. */
export function handleFrom(seed: string): string {
  let h = 0x811c9dc5;
  for (let i = 0; i < seed.length; i++) {
    h ^= seed.charCodeAt(i);
    h = Math.imul(h, 0x01000193);
  }
  const tag = (h >>> 0).toString(16).padStart(8, "0").slice(0, 6);
  return `citizen_${tag}`;
}

/** Two-letter avatar initials from a handle. */
export function initials(handle: string): string {
  const t = handle.replace(/^citizen_/, "");
  return t.slice(0, 2).toUpperCase();
}

/** Stable hue for an avatar, derived from a handle. */
export function hueFrom(seed: string): number {
  let h = 0;
  for (let i = 0; i < seed.length; i++) h = (h * 31 + seed.charCodeAt(i)) % 360;
  return h;
}

export function timeAgo(ts: number): string {
  const s = Math.max(1, Math.floor((Date.now() - ts) / 1000));
  if (s < 60) return `${s}s`;
  const m = Math.floor(s / 60);
  if (m < 60) return `${m}m`;
  const h = Math.floor(m / 60);
  if (h < 24) return `${h}h`;
  const d = Math.floor(h / 24);
  if (d < 7) return `${d}d`;
  return new Date(ts).toLocaleDateString(undefined, {
    month: "short",
    day: "numeric",
  });
}

export function shortId(id: string, keep = 6): string {
  const [ns, rest] = id.includes("/") ? id.split("/") : ["", id];
  if (!rest) return id;
  const tail = rest.length > keep ? `${rest.slice(0, keep)}…` : rest;
  return ns ? `${ns}/${tail}` : tail;
}

export function pct(n: number, total: number): number {
  if (total <= 0) return 0;
  return Math.round((n / total) * 100);
}
