import { createContext, useContext } from "react";
import type { Manifest } from "../types/Manifest/Manifest";

export const ManifestContext = createContext<Manifest | null>(null);

export const useManifest = () => {
  const manifest = useContext(ManifestContext);

  if (!manifest) {
    throw new Error("ManifestProvider missing");
  }

  return manifest;
};