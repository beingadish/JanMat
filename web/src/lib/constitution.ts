// Turns raw constitution keys/values into plain language a normal citizen
// understands. Keys mirror crates/janmat-api/src/routes/governance.rs.

export interface RuleView {
  key: string;
  title: string;
  desc: string;
  display: string;
}

type Unit = "percent" | "count" | "blocks";
const META: Record<
  string,
  { title: string; desc: string; unit?: Unit; suffix?: string }
> = {
  "quorum.regular": {
    title: "Everyday proposals",
    desc: "Share of votes an ordinary proposal needs to pass.",
    unit: "percent",
  },
  "quorum.constitutional": {
    title: "Rule changes",
    desc: "Changing the platform's own rules needs a supermajority.",
    unit: "percent",
  },
  "moderator.max_per_topic": {
    title: "Moderators per community",
    desc: "Most elected moderators a community can have at once.",
    unit: "count",
    suffix: "max",
  },
  "moderator.term_blocks": {
    title: "Moderator term",
    desc: "How long a moderator serves before facing re-election.",
    unit: "blocks",
  },
  "recall.petition_ratio": {
    title: "Removing a moderator",
    desc: "Members who must sign a petition to force a recall vote.",
    unit: "percent",
  },
  "referenda.support_ratio": {
    title: "Starting a public vote",
    desc: "Support needed to put an issue to the whole community.",
    unit: "percent",
  },
};

export function friendlyRule(key: string, value: string): RuleView {
  const m = META[key];
  const num = Number(value);
  let display = value;
  if (m?.unit === "percent" && !Number.isNaN(num)) {
    display = `${+(num * 100).toFixed(1)}%`;
  } else if (m?.unit === "blocks" && !Number.isNaN(num)) {
    display = `~${num} blocks`;
  } else if (m?.unit === "count") {
    display = m.suffix ? `${value} ${m.suffix}` : value;
  }
  return { key, title: m?.title ?? key, desc: m?.desc ?? "", display };
}
