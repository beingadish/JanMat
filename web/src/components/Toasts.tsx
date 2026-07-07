import { useEffect } from "react";
import { useCivic } from "../store/store";

export function Toasts() {
  const { state, dismissToast } = useCivic();
  return (
    <div className="toasts">
      {state.toasts.map((t) => (
        <ToastItem
          key={t.id}
          id={t.id}
          kind={t.kind}
          text={t.text}
          onDone={dismissToast}
        />
      ))}
    </div>
  );
}

function ToastItem({
  id,
  kind,
  text,
  onDone,
}: {
  id: number;
  kind: "ok" | "err" | "info";
  text: string;
  onDone: (id: number) => void;
}) {
  useEffect(() => {
    const t = setTimeout(() => onDone(id), 3800);
    return () => clearTimeout(t);
  }, [id, onDone]);
  return (
    <div className={`toast ${kind}`} onClick={() => onDone(id)}>
      {text}
    </div>
  );
}
