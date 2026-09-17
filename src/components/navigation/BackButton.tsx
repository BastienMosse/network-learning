import { ArrowLeft } from "lucide-react";
import "./BackButton.css";

export function BackButton({
  onClick,
  label = "Retour à l’exploration",
}: {
  onClick: () => void;
  label?: string;
}) {
  return (
    <button className="back-button" onClick={onClick}>
      <ArrowLeft size={15} /> {label}
    </button>
  );
}