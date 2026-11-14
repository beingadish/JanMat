import type { FeaturedCircle } from "../api/types";

interface CivicFeedsRailProps {
  activeFeed: string;
  setActiveFeed: (feed: string) => void;
  selectedCircle: string;
  setSelectedCircle: (circleId: string) => void;
  circles?: FeaturedCircle[];
  myBallotsCount?: number;
  passedCount?: number;
  turnoutPct?: number;
  consensusIndex?: number;
  weeklyChange?: string;
  precinctName?: string;
}

const DEFAULT_CIRCLES: FeaturedCircle[] = [
  { id: "governance", name: "JanMat Foundation", count: 0, icon: "gavel" },
  { id: "transit", name: "Transit Council", count: 0, icon: "directions_transit" },
  { id: "infrastructure", name: "Civic Infrastructure", count: 0, icon: "apartment" },
];

export function CivicFeedsRail({
  activeFeed,
  setActiveFeed,
  selectedCircle,
  setSelectedCircle,
  circles = DEFAULT_CIRCLES,
  myBallotsCount = 0,
  passedCount = 0,
  turnoutPct = 0,
  consensusIndex = 0,
  weeklyChange = "0.0",
  precinctName = "Precinct 42-North",
}: CivicFeedsRailProps) {
  return (
    <aside className="flex flex-col gap-6 w-full">
      {/* Civic Feeds Menu Card */}
      <div className="bg-[#131A29] rounded-2xl p-4 border border-[#222D42] shadow-md flex flex-col gap-3">
        <div className="flex items-center justify-between pb-1 border-b border-[#222D42]/60">
          <span className="text-[11px] font-bold uppercase tracking-wider text-[#64748B]">
            Civic Feeds
          </span>
          <span className="material-symbols-outlined text-[#64748B] text-[18px]">tune</span>
        </div>

        <nav className="flex flex-col gap-1.5">
          <button
            onClick={() => setActiveFeed("trending")}
            className={`w-full flex items-center justify-between px-3 py-2 rounded-full text-[13px] font-semibold transition-all ${
              activeFeed === "trending"
                ? "bg-[#1C263A] text-white border border-[#3E4C6D] shadow-sm"
                : "text-[#c7c4d8] hover:text-white hover:bg-[#1C263A] border border-transparent"
            }`}
          >
            <span className="flex items-center gap-2">
              <span className="material-symbols-outlined text-[#3B82F6] text-[18px]">
                trending_up
              </span>
              Trending Consensus
            </span>
            <span className="w-2 h-2 rounded-full bg-[#10B981] animate-pulse"></span>
          </button>

          <button
            onClick={() => setActiveFeed("my-ballots")}
            className={`w-full flex items-center justify-between px-3 py-2 rounded-full text-[13px] font-semibold transition-all ${
              activeFeed === "my-ballots"
                ? "bg-[#1C263A] text-white border border-[#3E4C6D] shadow-sm"
                : "text-[#c7c4d8] hover:text-white hover:bg-[#1C263A] border border-transparent"
            }`}
          >
            <span className="flex items-center gap-2">
              <span className="material-symbols-outlined text-[#64748B] text-[18px]">
                location_on
              </span>
              My Precinct Ballots
            </span>
            <span className="px-2 py-0.5 rounded-full bg-[#0A0E17] text-[#94A3B8] text-[11px] font-mono">
              {myBallotsCount}
            </span>
          </button>

          <button
            onClick={() => setActiveFeed("fast-track")}
            className={`w-full flex items-center justify-between px-3 py-2 rounded-full text-[13px] font-semibold transition-all ${
              activeFeed === "fast-track"
                ? "bg-[#1C263A] text-white border border-[#3E4C6D] shadow-sm"
                : "text-[#c7c4d8] hover:text-white hover:bg-[#1C263A] border border-transparent"
            }`}
          >
            <span className="flex items-center gap-2">
              <span className="material-symbols-outlined text-[#64748B] text-[18px]">
                bolt
              </span>
              Fast-Track Debates
            </span>
            <span className="px-2 py-0.5 rounded-full bg-[#0566D9]/40 text-[#adc6ff] text-[11px] font-bold">
              New
            </span>
          </button>

          <button
            onClick={() => setActiveFeed("passed")}
            className={`w-full flex items-center justify-between px-3 py-2 rounded-full text-[13px] font-semibold transition-all ${
              activeFeed === "passed"
                ? "bg-[#1C263A] text-white border border-[#3E4C6D] shadow-sm"
                : "text-[#c7c4d8] hover:text-white hover:bg-[#1C263A] border border-transparent"
            }`}
          >
            <span className="flex items-center gap-2">
              <span className="material-symbols-outlined text-[#64748B] text-[18px]">
                task_alt
              </span>
              Passed Resolutions
            </span>
            <span className="px-2 py-0.5 rounded-full bg-[#0A0E17] text-[#94A3B8] text-[11px] font-mono">
              {passedCount}
            </span>
          </button>
        </nav>
      </div>

      {/* Featured Circles Selector */}
      <div className="bg-[#131A29] rounded-2xl p-4 border border-[#222D42] shadow-md flex flex-col gap-3">
        <div className="flex items-center justify-between pb-1 border-b border-[#222D42]/60">
          <span className="text-[11px] font-bold uppercase tracking-wider text-[#64748B]">
            Featured Circles
          </span>
          <button
            onClick={() => setSelectedCircle("all")}
            className="text-[11px] font-semibold text-[#3B82F6] hover:underline bg-transparent border-none p-0 cursor-pointer"
          >
            View All
          </button>
        </div>

        <div className="flex flex-col gap-1.5">
          {circles.map((circle) => {
            const isSelected = selectedCircle === circle.id;
            return (
              <button
                key={circle.id}
                onClick={() => setSelectedCircle(isSelected ? "all" : circle.id)}
                className={`w-full flex items-center justify-between p-2 rounded-xl text-left transition-all ${
                  isSelected
                    ? "bg-[#1C263A] border border-[#3E4C6D]"
                    : "hover:bg-[#1C263A] border border-transparent"
                }`}
              >
                <div className="flex items-center gap-2.5 min-w-0">
                  <div
                    className={`w-8 h-8 rounded-full flex items-center justify-center shrink-0 ${
                      isSelected ? "bg-[#4F46E5] text-white" : "bg-[#262A34] text-[#DFE2EF]"
                    }`}
                  >
                    <span className="material-symbols-outlined text-[16px]">{circle.icon}</span>
                  </div>
                  <span className="text-[13px] font-semibold text-white truncate">
                    {circle.name}
                  </span>
                </div>
                <span
                  className={`px-2 py-0.5 rounded-full text-[11px] font-mono shrink-0 ${
                    isSelected
                      ? "bg-[#4F46E5] text-white font-bold"
                      : "bg-[#262A34] text-[#adc6ff]"
                  }`}
                >
                  {circle.count}
                </span>
              </button>
            );
          })}
        </div>
      </div>

      {/* Precinct Quick Stats Widget */}
      <div className="bg-[#131A29] rounded-2xl p-4 border border-[#222D42] shadow-md flex flex-col gap-3 relative overflow-hidden">
        <div className="flex items-center justify-between">
          <span className="text-[11px] font-bold uppercase tracking-wider text-[#64748B]">
            Node Status
          </span>
          <span className="inline-flex items-center gap-1.5 text-[11px] font-semibold text-[#10B981]">
            <span className="w-1.5 h-1.5 rounded-full bg-[#10B981] animate-pulse"></span>
            Syncing
          </span>
        </div>

        <div>
          <div className="font-['Plus_Jakarta_Sans'] font-bold text-[18px] text-white">
            {precinctName}
          </div>
          <div className="text-[12px] text-[#94A3B8]">Urban Metro Central Jurisdiction</div>
        </div>

        <div className="grid grid-cols-2 gap-2 pt-1">
          <div className="bg-[#0A0E17] p-3 rounded-xl border border-[#222D42] flex flex-col">
            <span className="text-[11px] font-semibold text-[#64748B]">Turnout</span>
            <span className="font-['Plus_Jakarta_Sans'] font-extrabold text-[20px] text-[#10B981]">
              {turnoutPct}%
            </span>
            <span className="text-[11px] text-[#94A3B8] mt-0.5">Verified Hash</span>
          </div>

          <div className="bg-[#0A0E17] p-3 rounded-xl border border-[#222D42] flex flex-col">
            <span className="text-[11px] font-semibold text-[#64748B]">Consensus</span>
            <span className="font-['Plus_Jakarta_Sans'] font-extrabold text-[20px] text-[#c3c0ff]">
              {consensusIndex}
              <span className="text-xs text-[#64748B] font-normal">/100</span>
            </span>
            <span className="text-[11px] text-[#10B981] mt-0.5">{weeklyChange} this week</span>
          </div>
        </div>

        {/* Micro Sparkline SVG */}
        <div className="pt-2">
          <div className="flex items-center justify-between text-[#64748B] text-[11px] font-semibold mb-1">
            <span>7-Day Deliberation Velocity</span>
            <span className="text-[#DFE2EF]">High Active</span>
          </div>
          <svg className="w-full h-8 overflow-visible text-[#3B82F6]" fill="none" viewBox="0 0 100 24">
            <path
              d="M 0,18 Q 15,22 30,12 T 60,8 T 85,15 T 100,4"
              fill="none"
              stroke="currentColor"
              strokeLinecap="round"
              strokeWidth="2.5"
            />
            <circle className="animate-ping" cx="100" cy="4" fill="#10B981" r="3" />
            <circle cx="100" cy="4" fill="#10B981" r="3" />
          </svg>
        </div>
      </div>
    </aside>
  );
}
