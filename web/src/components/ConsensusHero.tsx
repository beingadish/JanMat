interface ConsensusHeroProps {
  blockHeight?: number;
  precinctLabel?: string;
  onOpenDraft: () => void;
  onOpenRules: () => void;
}

export function ConsensusHero({
  blockHeight = 1,
  precinctLabel = "PRECINCT 42 NETWORK ACTIVE",
  onOpenDraft,
  onOpenRules,
}: ConsensusHeroProps) {
  return (
    <section className="relative w-full rounded-2xl bg-[#131A29] border border-[#222D42] overflow-hidden shadow-xl mb-4 sm:mb-6">
      {/* Ambient Lighting Orbs */}
      <div className="absolute -right-20 -top-24 w-72 h-72 rounded-full bg-[#4F46E5]/15 blur-3xl pointer-events-none"></div>

      <div className="relative z-10 p-4 sm:p-6 lg:p-8 flex flex-col md:flex-row md:items-center justify-between gap-4 sm:gap-6">
        <div className="max-w-2xl">
          <div className="flex items-center gap-2 mb-2 sm:mb-3 flex-wrap">
            <span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full bg-[#262A34] text-[#10B981] text-[10px] sm:text-[11px] font-bold tracking-wider">
              <span className="w-2 h-2 rounded-full bg-[#10B981] animate-pulse"></span>
              {precinctLabel}
            </span>
            <span className="text-[#64748B] text-[11px] sm:text-[13px] font-mono">
              • Ledger Block #{blockHeight.toLocaleString()}
            </span>
          </div>

          <h1 className="font-['Plus_Jakarta_Sans'] text-[22px] sm:text-[28px] lg:text-[36px] text-white tracking-tight font-extrabold leading-tight">
            Democracy in Real-Time: Public Pulse.
          </h1>
          <p className="text-[13px] sm:text-[15px] text-[#94A3B8] mt-1.5 sm:mt-2.5 leading-relaxed">
            Your cryptographic voice shapes verifiable consensus. Transparent deliberation,
            tamper-evident balloting, and direct community governance.
          </p>
        </div>

        <div className="flex flex-row sm:flex-col gap-2.5 shrink-0">
          <button
            onClick={onOpenDraft}
            className="inline-flex items-center justify-center gap-2 px-5 py-2.5 sm:px-6 sm:py-3 rounded-full bg-[#4F46E5] hover:bg-[#3B82F6] text-white text-[12px] sm:text-[13px] font-bold shadow-lg shadow-[#4F46E5]/30 transition-all active:scale-95"
          >
            <span className="material-symbols-outlined text-[16px] sm:text-[18px]">add_circle</span>
            <span>Draft a Motion</span>
          </button>
          <button
            onClick={onOpenRules}
            className="inline-flex items-center justify-center gap-2 px-4 py-2.5 sm:px-6 sm:py-3 rounded-full bg-[#262A34] hover:bg-[#1C263A] text-[#DFE2EF] text-[12px] sm:text-[13px] font-semibold border border-[#222D42] transition-colors"
          >
            <span className="material-symbols-outlined text-[16px] sm:text-[18px]">shield</span>
            <span>Explore Rules</span>
          </button>
        </div>
      </div>
    </section>
  );
}
