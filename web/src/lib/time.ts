import { useEffect, useState } from "react";

/** Format a remaining duration (ms) as HH:MM:SS, or "expired". */
export function fmtRemaining(ms: number): string {
  if (ms <= 0) return "expired";
  const s = Math.floor(ms / 1000);
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  const sec = s % 60;
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${pad(h)}:${pad(m)}:${pad(sec)}`;
}

/** A `Date.now()` that re-renders on an interval — for live countdowns. */
export function useNow(intervalMs = 1000): number {
  const [now, setNow] = useState(() => Date.now());
  useEffect(() => {
    const t = setInterval(() => setNow(Date.now()), intervalMs);
    return () => clearInterval(t);
  }, [intervalMs]);
  return now;
}

/**
 * Format relative elapsed time: f(Current Time - Created Time).
 * Evaluated locally inside citizen's browser without server network calls.
 * Re-renders reactively with useNow().
 */
export function formatRelativeTime(
  createdAt: string | number | undefined,
  nowMs: number = Date.now()
): string {
  if (!createdAt) return "just now";
  let createdMs = 0;
  if (typeof createdAt === "number") {
    // Check if seconds or milliseconds
    createdMs = createdAt < 1e12 ? createdAt * 1000 : createdAt;
  } else {
    createdMs = new Date(createdAt).getTime();
  }
  if (isNaN(createdMs) || createdMs <= 0) return "just now";

  const deltaMs = Math.max(0, nowMs - createdMs);
  const seconds = Math.floor(deltaMs / 1000);

  if (seconds < 5) return "just now";
  if (seconds < 60) return `${seconds}s ago`;
  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) return `${minutes}m ago`;
  const hours = Math.floor(seconds / 3600);
  if (hours < 24) return `${hours}h ago`;
  const days = Math.floor(hours / 24);
  if (days < 30) return `${days}d ago`;
  return new Date(createdMs).toLocaleDateString();
}
