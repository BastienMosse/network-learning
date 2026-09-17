import "./ThemeList.css";

import { ThemeCard } from "./ThemeCard";
import type { Manifest } from "../../types";

export function ThemeList({
    manifest,
    onTheme,
} : {
    manifest: Manifest,
    onTheme: (themeId: string) => void,
}) {
    return (
        <div className="theme-grid">
          {manifest.themes.map((theme) => (
            <ThemeCard
              key={theme.id}
              theme={theme}
              manifest={manifest}
              onClick={() => onTheme(theme.id)}
            />
          ))}
        </div>
    );
}