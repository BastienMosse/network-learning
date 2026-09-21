import "./CourseList.css";

import type { Course } from "../../types/Manifest/Course";
import { CourseCard } from "./CourseCard";

export function CourseList({
    courses,
    onCourse,
} : {
    courses: Course[],
    onCourse: (themeId: string, courseId: string) => void,
}) {
    return (
        <div className="course-grid">
          {courses.map((course, index) => (
            <CourseCard
              key={course.id}
              course={course}
              index={index}
              onClick={() => onCourse(course.themeId, course.id)}
            />
          ))}
        </div>
    );
}