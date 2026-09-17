import type { Theme } from './Theme'
import type { Course } from './Course'

export type Manifest = {
  themes: Theme[]
  courses: Course[]
  courseFiles: Record<string, string>
}