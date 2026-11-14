interface RulesModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function RulesModal({ isOpen, onClose }: RulesModalProps) {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-md animate-fadeIn">
      <div className="relative w-full max-w-xl bg-[#131A29] rounded-2xl border border-[#222D42] shadow-2xl p-6 flex flex-col gap-5 overflow-hidden">
        {/* Modal Header */}
        <div className="flex items-center justify-between border-b border-[#222D42] pb-3">
          <div className="flex items-center gap-2.5">
            <div className="w-8 h-8 rounded-full bg-[#4F46E5]/20 text-[#4F46E5] flex items-center justify-center">
              <span className="material-symbols-outlined text-[18px]">gavel</span>
            </div>
            <div>
              <h3 className="font-['Plus_Jakarta_Sans'] text-[18px] font-bold text-white">
                JanMat Constitution & Quorum Rules
              </h3>
              <p className="text-[12px] text-[#64748B]">Enacted at Genesis Block #1</p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="w-8 h-8 rounded-full bg-[#0A0E17] hover:bg-[#1C263A] text-[#94A3B8] hover:text-white flex items-center justify-center border border-[#222D42] transition-colors"
          >
            ✕
          </button>
        </div>

        {/* Rules Body */}
        <div className="flex flex-col gap-3 text-[13px] max-h-[60vh] overflow-y-auto pr-1">
          <div className="bg-[#0A0E17] p-3.5 rounded-xl border border-[#222D42] flex flex-col gap-1">
            <div className="flex items-center justify-between font-bold text-white">
              <span>Regular Motion Quorum</span>
              <span className="text-[#10B981] font-mono">50.1% (Simple Majority)</span>
            </div>
            <p className="text-[12px] text-[#94A3B8]">
              Standard civic proposals, budget allocations, and neighborhood mandates pass if &gt;50% of verified ballots vote Yea.
            </p>
          </div>

          <div className="bg-[#0A0E17] p-3.5 rounded-xl border border-[#222D42] flex flex-col gap-1">
            <div className="flex items-center justify-between font-bold text-white">
              <span>Constitutional Motion Quorum</span>
              <span className="text-[#c3c0ff] font-mono">67.0% (Two-Thirds Supermajority)</span>
            </div>
            <p className="text-[12px] text-[#94A3B8]">
              Changes to core governance parameters, voting thresholds, or moderator election systems require a two-thirds majority.
            </p>
          </div>

          <div className="bg-[#0A0E17] p-3.5 rounded-xl border border-[#222D42] flex flex-col gap-1">
            <div className="flex items-center justify-between font-bold text-white">
              <span>Moderator Term Length</span>
              <span className="text-[#3B82F6] font-mono">90 Blocks (~3 months)</span>
            </div>
            <p className="text-[12px] text-[#94A3B8]">
              Circle moderators hold elected terms for 90 blocks, subject to democratic recall.
            </p>
          </div>

          <div className="bg-[#0A0E17] p-3.5 rounded-xl border border-[#222D42] flex flex-col gap-1">
            <div className="flex items-center justify-between font-bold text-white">
              <span>Recall Petition Threshold</span>
              <span className="text-[#EF4444] font-mono">10.0% Petition Ratio</span>
            </div>
            <p className="text-[12px] text-[#94A3B8]">
              A recall vote against an active moderator is initiated when 10% of active voters in a circle cosign a recall petition.
            </p>
          </div>

          <div className="bg-[#0A0E17] p-3.5 rounded-xl border border-[#222D42] flex flex-col gap-1">
            <div className="flex items-center justify-between font-bold text-white">
              <span>Citizen Identity Nullifiers</span>
              <span className="text-[#10B981] font-mono">Zero-Knowledge Nullifiers</span>
            </div>
            <p className="text-[12px] text-[#94A3B8]">
              Every ballot is proved with a cryptographic nullifier that prevents double voting while ensuring zero linkage between identity and chosen stance.
            </p>
          </div>
        </div>

        {/* Modal Action */}
        <div className="flex items-center justify-end pt-2 border-t border-[#222D42]">
          <button
            onClick={onClose}
            className="px-5 py-2 rounded-full bg-[#4F46E5] hover:bg-[#3B82F6] text-white text-[13px] font-bold transition-all shadow-md active:scale-95"
          >
            Understood
          </button>
        </div>
      </div>
    </div>
  );
}
