import { parse } from "yaml";
import type { CourseContent, Manifest } from "../types/index";

export const fallbackManifest: Manifest = {
  themes: [],
  courses: [],
  courseFiles: {},
};

const base = import.meta.env.BASE_URL;

export async function loadManifest(): Promise<Manifest> {
  const response = await fetch(`${base}courses/manifest.json`);

  if (!response.ok) {
    return fallbackManifest;
  }

  return (await response.json()) as Manifest;
}

export async function loadCourse(path: string): Promise<CourseContent> {
  const url = path.startsWith("/") ? `${base}${path.slice(1)}` : path;
  const response = await fetch(url);

  if (!response.ok) {
    throw new Error("Impossible de charger ce cours");
  }

  return parse(await response.text()) as CourseContent;
}
