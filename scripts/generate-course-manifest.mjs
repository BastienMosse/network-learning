import { existsSync, readdirSync, readFileSync, writeFileSync } from 'node:fs'
import { join, relative, sep } from 'node:path'
import { parse } from 'yaml'

const root = join(process.cwd(), 'public', 'courses')
const manifestPath = join(root, 'manifest.json')
const manifest = JSON.parse(readFileSync(manifestPath, 'utf8'))
const discovered = []

function findCourseFiles(directory) {
  if (!existsSync(directory)) return
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    const entryPath = join(directory, entry.name)
    if (entry.isDirectory()) findCourseFiles(entryPath)
    if (entry.isFile() && entry.name === 'course.yaml') discovered.push(entryPath)
  }
}

findCourseFiles(root)

const courses = discovered.map((filePath) => {
  const course = parse(readFileSync(filePath, 'utf8'))
  if (!course || !course.id) {
    console.warn(`Skipping incomplete course file: ${filePath}`)
    return null
  }
  const relativePath = relative(join(process.cwd(), 'public'), filePath).split(sep).join('/')
  return {
    id: course.id,
    slug: course.slug || course.id,
    title: course.title,
    description: course.description,
    level: course.level || 'Tous niveaux',
    duration: course.duration || 'À votre rythme',
    themeId: course.themeId,
    color: course.color || 'coral',
    file: `/${relativePath}`,
  }
}).filter(Boolean)

const summaries = courses.map((course) => {
  const summary = { ...course }
  delete summary.file
  return summary
})
const discoveredById = new Map(summaries.map((course) => [course.id, course]))
const orderedCourses = [
  ...(manifest.courses || []).map((course) => discoveredById.get(course.id)).filter(Boolean),
  ...summaries.filter((course) => !(manifest.courses || []).some((existing) => existing.id === course.id)),
]
manifest.courses = orderedCourses
manifest.courseFiles = Object.fromEntries(courses.map(({ id, file }) => [id, file]))
const courseIdsByTheme = new Map()
for (const course of orderedCourses) {
  const ids = courseIdsByTheme.get(course.themeId) || []
  ids.push(course.id)
  courseIdsByTheme.set(course.themeId, ids)
}
manifest.themes = manifest.themes.map((theme) => ({
  ...theme,
  courseIds: courseIdsByTheme.get(theme.id) || [],
}))
writeFileSync(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`)
console.log(`Course manifest generated: ${courses.length} course(s)`)
