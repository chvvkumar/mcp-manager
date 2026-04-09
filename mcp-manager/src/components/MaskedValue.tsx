import { useState } from "react";
import { shouldMaskEntry } from "../lib/secrets";

interface MaskedValueProps { label: string; value: string; onChange: (value: string) => void; }

export function MaskedValue({ label, value, onChange }: MaskedValueProps) {
  const [revealed, setRevealed] = useState(false);
  const { masked, display } = shouldMaskEntry(label, value);
  return (
    <div className="flex items-center gap-2">
      <input type={masked && !revealed ? "password" : "text"}
        value={revealed || !masked ? value : display}
        onChange={(e) => onChange(e.target.value)}
        className="flex-1 px-2 py-1 bg-gray-700 border border-gray-600 rounded text-sm text-gray-200" />
      {masked && (
        <button onClick={() => setRevealed(!revealed)} className="text-gray-400 hover:text-white text-sm" title={revealed ? "Hide" : "Reveal"}>
          {revealed ? "Hide" : "Show"}
        </button>
      )}
    </div>
  );
}
