import type { BallotItem } from "../api/types";

interface AuditReceiptModalProps {
  ballot: BallotItem | null;
  onClose: () => void;
  blockHeight?: number;
}

export function AuditReceiptModal({ ballot, onClose, blockHeight = 849201 }: AuditReceiptModalProps) {
  if (!ballot) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-md animate-fadeIn">
      <div className="relative w-full max-w-lg bg-[#131A29] rounded-2xl border border-[#222D42] shadow-2xl p-6 flex flex-col gap-5 overflow-hidden">
        {/* Ambient Top Glow */}
        <div className="absolute -left-10 -top-10 w-44 h-44 rounded-full bg-[#10B981]/15 blur-3xl pointer-events-none"></div>

        {/* Modal Header */}
        <div className="flex items-center justify-between border-b border-[#222D42] pb-3">
          <div className="flex items-center gap-2.5">
            <div className="w-8 h-8 rounded-full bg-[#10B981]/20 text-[#10B981] flex items-center justify-center">
              <span className="material-symbols-outlined text-[18px]">verified</span>
            </div>
            <div>
              <h3 className="font-['Plus_Jakarta_Sans'] text-[17px] font-bold text-white">
                Ledger Cryptographic Audit
              </h3>
              <p className="text-[12px] text-[#64748B]">Proposal #{ballot.proposal_code} • Blake3 Audit Commitment</p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="w-8 h-8 rounded-full bg-[#0A0E17] hover:bg-[#1C263A] text-[#94A3B8] hover:text-white flex items-center justify-center border border-[#222D42] transition-colors"
          >
            ✕
          </button>
        </div>

        {/* Modal Content */}
        <div className="flex flex-col gap-4 text-[13px]">
          <div className="bg-[#0A0E17] p-3.5 rounded-xl border border-[#222D42] flex flex-col gap-1.5">
            <span className="text-[11px] font-bold text-[#64748B] uppercase tracking-wider">
              Blake3 State Root Hash
            </span>
            <div className="font-mono text-[12px] text-[#10B981] break-all bg-[#131A29] p-2.5 rounded-lg border border-[#222D42]">
              {ballot.blake3_hash}
            </div>
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div className="bg-[#0A0E17] p-3 rounded-xl border border-[#222D42] flex flex-col">
              <span className="text-[11px] text-[#64748B] font-semibold">Ledger Block</span>
              <span className="font-mono text-[14px] font-bold text-white mt-0.5">
                #{blockHeight.toLocaleString()}
              </span>
            </div>
            <div className="bg-[#0A0E17] p-3 rounded-xl border border-[#222D42] flex flex-col">
              <span className="text-[11px] text-[#64748B] font-semibold">Consensus Quorum</span>
              <span className="font-mono text-[14px] font-bold text-[#10B981] mt-0.5">
                {ballot.tally.quorum_pct}% (MET)
              </span>
            </div>
          </div>

          <div className="bg-[#0A0E17] p-3.5 rounded-xl border border-[#222D42] flex flex-col gap-2">
            <span className="text-[11px] font-bold text-[#64748B] uppercase tracking-wider">
              Zero-Knowledge Verification Status
            </span>
            <div className="flex items-center justify-between text-[12px]">
              <span className="text-[#94A3B8]">Anti-Sybil Proof:</span>
              <span className="text-[#10B981] font-semibold flex items-center gap-1">
                <span className="material-symbols-outlined text-[14px]">check_circle</span>
                Groth16 Valid
              </span>
            </div>
            <div className="flex items-center justify-between text-[12px]">
              <span className="text-[#94A3B8]">Identity Nullifier Seed:</span>
              <span className="font-mono text-[#adc6ff] text-[11px]">0x8e2f...b301</span>
            </div>
            <div className="flex items-center justify-between text-[12px]">
              <span className="text-[#94A3B8]">Ballot Commitment:</span>
              <span className="font-mono text-[#adc6ff] text-[11px]">Blake3-256 HMAC</span>
            </div>
          </div>
        </div>

        {/* Modal Action */}
        <div className="flex items-center justify-end pt-2 border-t border-[#222D42]">
          <button
            onClick={onClose}
            className="px-5 py-2 rounded-full bg-[#4F46E5] hover:bg-[#3B82F6] text-white text-[13px] font-bold transition-all shadow-md active:scale-95"
          >
            Done
          </button>
        </div>
      </div>
    </div>
  );
}
