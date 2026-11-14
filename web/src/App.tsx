import { useEffect, useRef, useState } from "react";
import { CivicTopBar } from "./components/CivicTopBar";
import { Toasts } from "./components/Toasts";
import { useNow } from "./lib/time";
import { StoreProvider, useCivic } from "./store/store";
import { CivicPulseView } from "./views/CivicPulseView";
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
  const { state, setView } = useCivic();
  const { view } = state;

  const [activeTab, setActiveTab] = useState("live-ballots");
  const [searchQuery, setSearchQuery] = useState("");
  const [isDraftModalOpen, setIsDraftModalOpen] = useState(false);
  const [isRulesModalOpen, setIsRulesModalOpen] = useState(false);

  // If user switches tab from header
  const handleTabChange = (tab: string) => {
    setActiveTab(tab);
    if (tab === "rules-verification") {
      setIsRulesModalOpen(true);
    } else if (tab === "governance") {
      setView("governance");
    } else {
      setView("feed");
    }
  };

  return (
    <div className="bg-[#0A0E17] min-h-screen text-[#DFE2EF] font-['Plus_Jakarta_Sans'] antialiased flex flex-col selection:bg-[#4F46E5] selection:text-white">
      <SessionWatcher />

      {/* Modern Dark Glassmorphic Header */}
      <CivicTopBar
        activeTab={activeTab}
        setActiveTab={handleTabChange}
        searchQuery={searchQuery}
        setSearchQuery={setSearchQuery}
        onOpenDraft={() => setIsDraftModalOpen(true)}
        onOpenRules={() => setIsRulesModalOpen(true)}
      />

      {/* Main Content Area */}
      <main className="w-full pt-24 pb-16 px-4 lg:px-8 max-w-7xl mx-auto flex-1">
        {view === "identity" ? (
          <div className="max-w-2xl mx-auto bg-[#131A29] p-6 sm:p-8 rounded-2xl border border-[#222D42] shadow-xl">
            <div className="flex items-center justify-between mb-6 pb-3 border-b border-[#222D42]">
              <h2 className="text-[20px] font-bold text-white flex items-center gap-2">
                <span className="material-symbols-outlined text-[#4F46E5]">fingerprint</span>
                Citizen Identity & Verification
              </h2>
              <button
                onClick={() => setView("feed")}
                className="px-4 py-1.5 rounded-full bg-[#1C263A] hover:bg-[#262A34] text-[#94A3B8] text-[13px] font-semibold transition-colors"
              >
                Back to Feed
              </button>
            </div>
            <Identity />
          </div>
        ) : view === "governance" ? (
          <div className="max-w-4xl mx-auto bg-[#131A29] p-6 sm:p-8 rounded-2xl border border-[#222D42] shadow-xl">
            <div className="flex items-center justify-between mb-6 pb-3 border-b border-[#222D42]">
              <h2 className="text-[20px] font-bold text-white flex items-center gap-2">
                <span className="material-symbols-outlined text-[#4F46E5]">gavel</span>
                Constitution & Governance
              </h2>
              <button
                onClick={() => setView("feed")}
                className="px-4 py-1.5 rounded-full bg-[#1C263A] hover:bg-[#262A34] text-[#94A3B8] text-[13px] font-semibold transition-colors"
              >
                Back to Feed
              </button>
            </div>
            <Governance />
          </div>
        ) : (
          <CivicPulseView
            activeTab={activeTab}
            searchQuery={searchQuery}
            isDraftModalOpen={isDraftModalOpen}
            setIsDraftModalOpen={setIsDraftModalOpen}
            isRulesModalOpen={isRulesModalOpen}
            setIsRulesModalOpen={setIsRulesModalOpen}
          />
        )}
      </main>

      <Toasts />
    </div>
  );
}

export default function App() {
  return (
    <StoreProvider>
      <Shell />
    </StoreProvider>
  );
}
