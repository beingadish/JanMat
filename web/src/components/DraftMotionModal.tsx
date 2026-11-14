import { useState } from "react";
import type { BallotItem } from "../api/types";

interface DraftMotionModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: (title: string, summary: string, circleId: string, authorName: string) => Promise<BallotItem | null>;
}

export function DraftMotionModal({ isOpen, onClose, onSubmit }: DraftMotionModalProps) {
  const [title, setTitle] = useState("");
  const [summary, setSummary] = useState("");
  const [circleId, setCircleId] = useState("transit");
  const [authorName, setAuthorName] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!title.trim() || !summary.trim()) {
      setError("Please fill in both the motion title and deliberation thesis.");
      return;
    }

    setLoading(true);
    setError(null);
    try {
      const res = await onSubmit(title.trim(), summary.trim(), circleId, authorName.trim() || "Citizen Sponsor");
      if (res) {
        setTitle("");
        setSummary("");
        onClose();
      }
    } catch (err) {
      setError((err as Error).message || "Failed to publish motion");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-md animate-fadeIn">
      <div className="relative w-full max-w-xl bg-[#131A29] rounded-2xl border border-[#222D42] shadow-2xl p-6 flex flex-col gap-5 overflow-hidden">
        {/* Ambient Glow */}
        <div className="absolute -right-12 -top-12 w-48 h-48 rounded-full bg-[#4F46E5]/20 blur-3xl pointer-events-none"></div>

        {/* Modal Header */}
        <div className="flex items-center justify-between border-b border-[#222D42] pb-3">
          <div className="flex items-center gap-2.5">
            <div className="w-8 h-8 rounded-full bg-[#4F46E5]/20 text-[#4F46E5] flex items-center justify-center">
              <span className="material-symbols-outlined text-[18px]">add_circle</span>
            </div>
            <div>
              <h3 className="font-['Plus_Jakarta_Sans'] text-[18px] font-bold text-white">
                Draft Civic Motion
              </h3>
              <p className="text-[12px] text-[#64748B]">Publish directly to the Precinct 42 ledger</p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="w-8 h-8 rounded-full bg-[#0A0E17] hover:bg-[#1C263A] text-[#94A3B8] hover:text-white flex items-center justify-center border border-[#222D42] transition-colors"
          >
            ✕
          </button>
        </div>

        {error && (
          <div className="p-3 rounded-xl bg-[#EF4444]/15 border border-[#EF4444]/30 text-[#EF4444] text-[13px]">
            {error}
          </div>
        )}

        {/* Form Body */}
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <div>
            <label className="block text-[12px] font-semibold text-[#94A3B8] uppercase tracking-wider mb-1.5">
              Target Circle / Domain
            </label>
            <select
              value={circleId}
              onChange={(e) => setCircleId(e.target.value)}
              className="w-full bg-[#0A0E17] border border-[#222D42] rounded-xl px-4 py-2.5 text-[14px] text-white focus:outline-none focus:border-[#4F46E5]"
            >
              <option value="transit">Transit Council</option>
              <option value="it-ethics">Municipal IT & AI Ethics</option>
              <option value="infrastructure">Civic Infrastructure</option>
              <option value="energy">Clean Energy & Water</option>
              <option value="healthcare">Healthcare Mandates</option>
            </select>
          </div>

          <div>
            <label className="block text-[12px] font-semibold text-[#94A3B8] uppercase tracking-wider mb-1.5">
              Motion Title / Core Question
            </label>
            <input
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="e.g. Should municipal electric buses receive dedicated lanes during rush hours?"
              className="w-full bg-[#0A0E17] border border-[#222D42] rounded-xl px-4 py-2.5 text-[14px] text-white placeholder:text-[#64748B] focus:outline-none focus:border-[#4F46E5]"
            />
          </div>

          <div>
            <label className="block text-[12px] font-semibold text-[#94A3B8] uppercase tracking-wider mb-1.5">
              Deliberation Thesis & Justification
            </label>
            <textarea
              rows={4}
              value={summary}
              onChange={(e) => setSummary(e.target.value)}
              placeholder="State the objective need, fiscal feasibility, and potential community trade-offs..."
              className="w-full bg-[#0A0E17] border border-[#222D42] rounded-xl px-4 py-2.5 text-[14px] text-white placeholder:text-[#64748B] focus:outline-none focus:border-[#4F46E5] resize-none"
            />
          </div>

          <div>
            <label className="block text-[12px] font-semibold text-[#94A3B8] uppercase tracking-wider mb-1.5">
              Author / Sponsor Handle
            </label>
            <input
              type="text"
              value={authorName}
              onChange={(e) => setAuthorName(e.target.value)}
              placeholder="Your name or handle (e.g. Aadarsh)"
              className="w-full bg-[#0A0E17] border border-[#222D42] rounded-xl px-4 py-2.5 text-[14px] text-white placeholder:text-[#64748B] focus:outline-none focus:border-[#4F46E5]"
            />
          </div>

          {/* Modal Actions */}
          <div className="flex items-center justify-end gap-3 pt-3 border-t border-[#222D42]">
            <button
              type="button"
              onClick={onClose}
              className="px-5 py-2.5 rounded-full bg-[#0A0E17] hover:bg-[#1C263A] text-[#94A3B8] text-[13px] font-semibold transition-colors"
            >
              Cancel
            </button>
            <button
              type="submit"
              disabled={loading}
              className="px-6 py-2.5 rounded-full bg-[#4F46E5] hover:bg-[#3B82F6] text-white text-[13px] font-bold shadow-lg shadow-[#4F46E5]/30 transition-all active:scale-95 disabled:opacity-50"
            >
              {loading ? "Publishing..." : "Submit to Ledger"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
