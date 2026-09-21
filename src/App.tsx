import { useEffect, useState } from "react";
import { RouterProvider } from "react-router-dom";

import { router } from "./router/router";

import type { Manifest } from "./courses/types/Manifest/Manifest";
import { loadManifest, fallbackManifest } from "./courses/services/courseService";
import { ManifestContext } from "./courses/contexts/ManifestContext";
import { ThemeContext } from "./courses/contexts/ThemeContext";

export default function App() {
  const [manifest, setManifest] = useState<Manifest>(fallbackManifest);
  const [loading, setLoading] = useState(true);
  const [dark, setDark] = useState(false);

  useEffect(() => {
    loadManifest()
      .then(setManifest)
      .finally(() => setLoading(false));
  }, []);

  const toggleTheme = () => {
    setDark((current) => !current);
  };

  return (
    <ThemeContext.Provider value={{ dark, toggleTheme }}>
      <ManifestContext.Provider value={manifest}>
        {loading ? (
          <div className="loading">
            Chargement du laboratoire<span>...</span>
          </div>
        ) : (
          <RouterProvider router={router} />
        )}
      </ManifestContext.Provider>
    </ThemeContext.Provider>
  );
}