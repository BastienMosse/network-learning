import { parse } from "yaml";
import type { CourseContent, Manifest } from "../types/index";

export const fallbackManifest: Manifest = {
  themes: [],
  courses: [],
  courseFiles: {},
};

export async function loadManifest(): Promise<Manifest> {
  const response = await fetch("/courses/manifest.json");

  if (!response.ok) {
    return fallbackManifest;
  }

  return (await response.json()) as Manifest;
}

export async function loadCourse(path: string): Promise<CourseContent> {
  console.log("path :", path);

  const response = await fetch(path);

  if (!response.ok) {
    throw new Error("Impossible de charger ce cours");
  }

  return parse(await response.text()) as CourseContent;
}
