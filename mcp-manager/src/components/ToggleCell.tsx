import type { BindingStatus } from "../types";

interface ToggleCellProps {
  status: BindingStatus | "not_present";
  onToggle: () => void;
  onAdd: () => void;
}

export function ToggleCell({ status, onToggle, onAdd }: ToggleCellProps) {
  if (status === "not_present") {
    return (
      <td className="px-3 py-2 text-center">
        <button onClick={onAdd} className="text-gray-500 hover:text-blue-400 text-lg" title="Add to this tool">+</button>
      </td>
    );
  }
  const enabled = status === "enabled";
  return (
    <td className="px-3 py-2 text-center">
      <button onClick={onToggle}
        className={`relative inline-flex h-5 w-9 items-center rounded-full transition-colors ${enabled ? "bg-green-500" : "bg-gray-600"}`}>
        <span className={`inline-block h-3.5 w-3.5 transform rounded-full bg-white transition-transform ${enabled ? "translate-x-4.5" : "translate-x-0.5"}`} />
      </button>
    </td>
  );
}
