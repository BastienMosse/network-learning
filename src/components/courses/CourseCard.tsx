import { BookOpen, Clock3, Layers3, ArrowUpRight } from "lucide-react";
import "./CourseCard.css";

import type { Course } from "../../types/Manifest/Course";

export function CourseCard({
  course,
  index,
  onClick,
}: {
  course: Course;
  index: number;
  onClick: () => void;
}) {
  return (
    <button className={`course-card ${course.color}`} onClick={onClick}>
      <span className="course-number">0{index + 1}</span>
      <span className="course-icon">
        {course.color === "blue" ? (
          <Layers3 size={22} />
        ) : (
          <BookOpen size={22} />
        )}
      </span>
      <h3>{course.title}</h3>
      <p>{course.description}</p>
      <span className="course-meta">
        <span>{course.level}</span>
        <span>
          <Clock3 size={14} /> {course.duration}
        </span>
      </span>
      <span className="card-arrow">
        <ArrowUpRight size={18} />
      </span>
    </button>
  );
}