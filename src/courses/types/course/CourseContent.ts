import type { Section } from "./section/Section"

export type CourseContent = {
  id: string
  slug: string
  title: string
  description: string
  level: string
  duration: string
  themeId: string
  color: string
  objectives: string[]
  sections: Section[]
}