import { fmtRemaining, useNow } from "../lib/time";
import { useCivic } from "../store/store";
import { Avatar, Button, Icon, VerifiedBadge } from "./ui";

export function TopBar() {
  const { state, setTopic, setView, logout } = useCivic();
  const { identity } = state;
  const now = useNow(1000);
  const remaining = identity ? identity.expiresAt - now : 0;
  const low = remaining > 0 && remaining < 60 * 60 * 1000; // < 1h

  return (
    <header className="topbar">
      <button className="brand" onClick={() => setTopic(null)}>
        <span className="logo">◈</span>
        <span className="brand-name">JanMat</span>
        <span className="brand-tag">civic ledger</span>
      </button>

      <div className="topbar-right">
        {identity ? (
          <div className="user-chip">
            <span
              className={`pwd-timer ${low ? "low" : ""}`}
              title="Your password / session expires in"
            >
              <Icon name="clock" size={14} /> {fmtRemaining(remaining)}
            </span>
            <button className="user-btn" title="View your identity" onClick={() => setView("identity")}>
              <Avatar seed={identity.handle} size={28} />
              <span className="user-name">@{identity.username}</span>
              <VerifiedBadge verified={identity.verified} />
            </button>
            <button className="iconbtn" title="Sign out" onClick={logout}>
              <Icon name="back" />
            </button>
          </div>
        ) : (
          <Button onClick={() => setView("identity")}>
            <Icon name="id" size={16} /> Log in / Register
          </Button>
        )}
      </div>
    </header>
  );
}
