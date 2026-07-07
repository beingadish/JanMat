import {
  type ButtonHTMLAttributes,
  type ReactNode,
  useState,
} from "react";
import { hueFrom, initials } from "../lib/civic";

export function Avatar({ seed, size = 36 }: { seed: string; size?: number }) {
  const hue = hueFrom(seed);
  return (
    <div
      className="avatar"
      style={{
        width: size,
        height: size,
        background: `linear-gradient(140deg, hsl(${hue} 65% 52%), hsl(${(hue + 45) % 360} 60% 42%))`,
        fontSize: size * 0.36,
      }}
      aria-hidden
    >
      {initials(seed.startsWith("citizen_") ? seed : `citizen_${seed}`)}
    </div>
  );
}

/** Reddit-style community icon: rounded square with the slug's initial(s). */
export function CommunityTile({ slug, size = 32 }: { slug: string; size?: number }) {
  const hue = hueFrom("j/" + slug);
  const label = slug.replace(/[^a-z0-9]/gi, "").slice(0, 1).toUpperCase() || "?";
  return (
    <div
      className="ctile"
      style={{
        width: size,
        height: size,
        background: `hsl(${hue} 55% 46%)`,
        fontSize: size * 0.5,
      }}
      aria-hidden
    >
      {label}
    </div>
  );
}

export function VerifiedBadge({ verified }: { verified: boolean }) {
  return verified ? (
    <span className="badge verified" title="Verified citizen">
      <Icon name="verify" /> Verified
    </span>
  ) : (
    <span className="badge unverified">guest</span>
  );
}

type Variant = "primary" | "ghost" | "subtle" | "danger";
export function Button({
  children,
  variant = "primary",
  loading,
  className = "",
  ...rest
}: {
  children: ReactNode;
  variant?: Variant;
  loading?: boolean;
} & ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button
      className={`btn ${variant} ${className}`}
      disabled={loading || rest.disabled}
      {...rest}
    >
      {loading ? <span className="spinner" /> : children}
    </button>
  );
}

/** Collapsible raw-API panel — shows this action really hit janmat-api. */
export function Receipt({ label, data }: { label: string; data: unknown }) {
  const [open, setOpen] = useState(false);
  return (
    <div className="receipt">
      <button className="receipt-toggle" onClick={() => setOpen((o) => !o)}>
        <Icon name="code" /> {label} {open ? "▾" : "▸"}
      </button>
      {open && <pre className="json">{JSON.stringify(data, null, 2)}</pre>}
    </div>
  );
}

export function Icon({ name, size = 18 }: { name: keyof typeof PATHS; size?: number }) {
  return (
    <svg viewBox="0 0 24 24" width={size} height={size} aria-hidden className="icon">
      <path d={PATHS[name]} />
    </svg>
  );
}

const PATHS = {
  home: "M3 11l9-8 9 8v9a1 1 0 0 1-1 1h-5v-6H9v6H4a1 1 0 0 1-1-1z",
  plus: "M11 5h2v6h6v2h-6v6h-2v-6H5v-2h6z",
  poll: "M4 13h3v7H4zM10.5 4h3v16h-3zM17 9h3v11h-3z",
  gov: "M12 2 2 7v2h20V7L12 2zM4 10v8H2v2h20v-2h-2v-8h-2v8h-3v-8h-2v8h-2v-8H9v8H6v-8H4z",
  id: "M12 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8zm0 2c-4 0-8 2-8 5v1h16v-1c0-3-4-5-8-5z",
  verify: "M9 16.2 4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4z",
  code: "M9.4 16.6 4.8 12l4.6-4.6L8 6l-6 6 6 6zM14.6 16.6 19.2 12l-4.6-4.6L16 6l6 6-6 6z",
  back: "M15.4 7.4 14 6l-6 6 6 6 1.4-1.4L10.8 12z",
  community:
    "M12 3a4 4 0 1 0 0 8 4 4 0 0 0 0-8zm0 10c-4.4 0-8 2.2-8 5v3h16v-3c0-2.8-3.6-5-8-5z",
  clock:
    "M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm1 10V7h-2v6l4.5 2.7 1-1.7z",
  copy: "M16 1H4a2 2 0 0 0-2 2v12h2V3h12zm3 4H8a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h11a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2zm0 16H8V7h11z",
} as const;
