import { useCivic } from "../store/store";

interface CivicTopBarProps {
  onOpenDraft: () => void;
  onOpenRules: () => void;
  activeTab: string;
  setActiveTab: (tab: string) => void;
  searchQuery: string;
  setSearchQuery: (query: string) => void;
  blockHeight?: number;
}

export function CivicTopBar({
  onOpenDraft,
  onOpenRules,
  activeTab,
  setActiveTab,
  searchQuery,
  setSearchQuery,
  blockHeight = 1,
}: CivicTopBarProps) {
  const { state, setView } = useCivic();
  const { identity } = state;

  return (
    <header className="fixed top-0 left-0 right-0 z-50 bg-[#131A29]/90 backdrop-blur-xl border-b border-[#222D42]">
      <div className="max-w-7xl mx-auto px-4 lg:px-8 h-20 flex items-center justify-between gap-4">
        {/* Brand Logo & Title */}
        <div className="flex items-center gap-6">
          <button
            onClick={() => {
              setActiveTab("live-ballots");
              setView("feed");
            }}
            className="flex items-center gap-3 text-left group bg-transparent border-none p-0 cursor-pointer"
          >
            <div className="w-10 h-10 rounded-full bg-[#4F46E5] flex items-center justify-center text-white shadow-[0_0_16px_rgba(79,70,229,0.4)] group-hover:scale-105 transition-transform shrink-0">
              <span className="material-symbols-outlined text-[20px]">how_to_vote</span>
            </div>
            <div className="flex flex-col">
              <span className="font-['Plus_Jakarta_Sans'] font-extrabold text-[18px] leading-tight tracking-tight text-white flex items-center gap-1.5">
                JanMat{" "}
                <span className="text-[11px] font-bold text-[#adc6ff] px-2 py-0.5 rounded bg-[#262A34] border border-[#222D42]">
                  जनमत
                </span>
              </span>
              <span className="text-[11px] font-medium text-[#64748B]">Civic Consensus Engine</span>
            </div>
          </button>

          {/* Desktop Nav Items */}
          <nav className="hidden xl:flex items-center gap-2">
            <button
              onClick={() => setActiveTab("explore-circles")}
              className={`px-3 py-1.5 rounded-full text-[13px] font-semibold transition-colors border ${
                activeTab === "explore-circles"
                  ? "bg-[#1C263A] text-white border-[#3E4C6D]"
                  : "text-[#c7c4d8] hover:text-white hover:bg-[#1C263A] border-transparent"
              }`}
            >
              Explore Circles
            </button>
            <button
              onClick={() => setActiveTab("live-ballots")}
              className={`px-3 py-1.5 rounded-full text-[13px] font-semibold transition-colors flex items-center gap-2 border ${
                activeTab === "live-ballots"
                  ? "bg-[#1C263A] text-white border-[#3E4C6D]"
                  : "text-[#c7c4d8] hover:text-white hover:bg-[#1C263A] border-transparent"
              }`}
            >
              <span className="w-2 h-2 rounded-full bg-[#10B981] animate-pulse"></span>
              Live Ballots
            </button>
            <button
              onClick={() => setActiveTab("deliberation-rooms")}
              className={`px-3 py-1.5 rounded-full text-[13px] font-semibold transition-colors border ${
                activeTab === "deliberation-rooms"
                  ? "bg-[#1C263A] text-white border-[#3E4C6D]"
                  : "text-[#c7c4d8] hover:text-white hover:bg-[#1C263A] border-transparent"
              }`}
            >
              Deliberation Rooms
            </button>
            <button
              onClick={onOpenRules}
              className={`px-3 py-1.5 rounded-full text-[13px] font-semibold transition-colors border ${
                activeTab === "rules-verification"
                  ? "bg-[#1C263A] text-white border-[#3E4C6D]"
                  : "text-[#c7c4d8] hover:text-white hover:bg-[#1C263A] border-transparent"
              }`}
            >
              Rules & Verification
            </button>
          </nav>
        </div>

        {/* Global Search Bar */}
        <div className="hidden md:flex flex-1 max-w-md mx-4">
          <div className="relative w-full">
            <span className="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-[#64748B] text-[18px]">
              search
            </span>
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search topics, citizen bills, circles..."
              className="w-full bg-[#0A0E17] border border-[#222D42] rounded-full pl-10 pr-4 py-2 text-[13px] text-white placeholder:text-[#64748B] focus:outline-none focus:border-[#4F46E5] focus:ring-1 focus:ring-[#4F46E5] transition-all"
            />
          </div>
        </div>

        {/* Right CTA Actions */}
        <div className="flex items-center gap-3">
          {/* Live Node Pill */}
          <div className="hidden sm:inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-[#181B25] border border-[#222D42] text-[11px] font-mono text-[#4EDEAF]">
            <span className="w-1.5 h-1.5 rounded-full bg-[#10B981] animate-pulse"></span>
            <span>#{blockHeight.toLocaleString()}</span>
          </div>

          <button
            onClick={onOpenDraft}
            className="flex items-center gap-1.5 px-4 py-2 rounded-full bg-[#4F46E5] hover:bg-[#3B82F6] text-white text-[13px] font-semibold border border-[#3E4C6D] transition-all shadow-[0_0_12px_rgba(79,70,229,0.3)] active:scale-95"
          >
            <span className="material-symbols-outlined text-[16px]">edit_document</span>
            <span className="hidden sm:inline">Draft Proposal</span>
            <span className="sm:hidden">Draft</span>
          </button>

          <div className="h-6 w-[1px] bg-[#222D42] hidden sm:block"></div>

          {/* Citizen Profile or Login Trigger */}
          <button
            onClick={() => setView(state.view === "identity" ? "feed" : "identity")}
            className="flex items-center gap-2 p-1 rounded-full hover:bg-[#1C263A] transition-colors border border-transparent hover:border-[#222D42]"
            title={identity ? `Logged in as @${identity.handle}` : "Register or Log in"}
          >
            <div className="w-8 h-8 rounded-full bg-[#4F46E5] text-white flex items-center justify-center font-bold text-[13px]">
              {identity ? (
                identity.handle.slice(0, 2).toUpperCase()
              ) : (
                <span className="material-symbols-outlined text-[18px]">person</span>
              )}
            </div>
            {identity && (
              <span className="hidden lg:inline text-[13px] font-semibold text-white max-w-[100px] truncate">
                @{identity.handle}
              </span>
            )}
          </button>
        </div>
      </div>
    </header>
  );
}
