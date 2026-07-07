import { useEffect, useRef } from "react";
import { AboutRail } from "./components/AboutRail";
import { CommunityNav } from "./components/CommunityNav";
import { Toasts } from "./components/Toasts";
import { TopBar } from "./components/TopBar";
import { useNow } from "./lib/time";
import { StoreProvider, useCivic } from "./store/store";
import { Feed } from "./views/Feed";
import { Governance } from "./views/Governance";
import { Identity } from "./views/Identity";

/** Auto-logs-out the moment the 24h password expires. */
function SessionWatcher() {
  const { state, expireSession } = useCivic();
  const now = useNow(1000);
  const fired = useRef(false);
  useEffect(() => {
    if (state.identity && now >= state.identity.expiresAt) {
      if (!fired.current) {
        fired.current = true;
        expireSession();
      }
    } else {
      fired.current = false;
    }
  }, [now, state.identity, expireSession]);
  return null;
}

function Shell() {
  const { state } = useCivic();
  const { view } = state;
  return (
    <>
      <SessionWatcher />
      <TopBar />
      <div className="layout">
        <CommunityNav />
        <main className="center" key={view}>
          {view === "feed" && <Feed />}
          {view === "governance" && <Governance />}
          {view === "identity" && <Identity />}
        </main>
        {view === "feed" ? <AboutRail /> : <div className="rail-empty" />}
      </div>
      <Toasts />
    </>
  );
}

export default function App() {
  return (
    <StoreProvider>
      <Shell />
    </StoreProvider>
  );
}
