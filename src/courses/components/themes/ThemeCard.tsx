import { ArrowRight, ArrowUpRight } from "lucide-react";
import "./ThemeCard.css";

import type { Theme } from "../../types/Manifest/Theme";
import type { Manifest } from "../../types/Manifest/Manifest";

export function ThemeCard({
  theme,
  manifest,
  onClick,
}: {
  theme: Theme;
  manifest: Manifest;
  onClick: () => void;
}) {
  const count = manifest.courses.filter(
    (course) => course.themeId === theme.id,
  ).length;
  
  return (
    <button className={`theme-card ${theme.accent}`} onClick={onClick}>
      <div className="theme-card-top">
        <span>{theme.eyebrow}</span>
        <ArrowUpRight />
      </div>
      <strong>{theme.title}</strong>
      <p>{theme.description}</p>
      <span className="theme-card-bottom">
        {count ? `${count} cours` : "Bientôt"} <ArrowRight size={15} />
      </span>
    </button>
  );
}
