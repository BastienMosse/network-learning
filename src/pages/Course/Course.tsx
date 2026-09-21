import { ArrowLeft, ArrowRight, Check, ChevronDown } from "lucide-react";
import { useNavigate, useParams } from "react-router-dom";
import { useEffect, useState } from "react";
import "./Course.css";

import { useManifest } from "../../courses/contexts/ManifestContext";
import { BackButton } from "../../courses/components/navigation/BackButton";
import { loadCourse } from "../../courses/services/courseService";
import type { CourseContent } from "../../courses/types/course/CourseContent";
import { BlockRenderer } from "../../courses/components/course/BlockRenderer";

export function Course() {
  const { themeId, courseId } = useParams();
  const navigate = useNavigate();
  const manifest = useManifest();

  const [course, setCourse] = useState<CourseContent>();
  const [loading, setLoading] = useState(true);
  const [sectionIndex, setSectionIndex] = useState(0);

  useEffect(() => {
    setLoading(true);
    setCourse(undefined);
    setSectionIndex(0);

    loadCourse(manifest.courseFiles[courseId!])
      .then(setCourse)
      .finally(() => setLoading(false));
  }, [courseId, manifest.courseFiles]);

  if (loading || !course) {
    return (
      <div className="loading">
        Chargement du cours<span>...</span>
      </div>
    );
  }

  const section = course.sections[sectionIndex];
  const progress = ((sectionIndex + 1) / course.sections.length) * 100;

  const onBack = () => { navigate(`/courses/theme/${themeId}`) };
  const openCourse = (id: string) => { navigate(`/courses/theme/${themeId}/course/${id}`); };
  const previousSection = () => {
    if (sectionIndex > 0) {
      setSectionIndex(sectionIndex - 1)
    } else if (previousCourse) {
      openCourse(previousCourse)
    }
  };
  const nextSection = () => {
    if (sectionIndex < course.sections.length - 1) {
      setSectionIndex(sectionIndex + 1)
    } else if (nextCourse) {
      openCourse(nextCourse)
    }
  };

  const theme = manifest.themes.find((theme) => theme.id === themeId)!;
  const courseIndex = theme.courseIds.findIndex((course) => course === courseId);
  const previousCourse = courseIndex > 0 ? theme.courseIds[courseIndex - 1] : undefined;
  const nextCourse = courseIndex < theme.courseIds.length - 1 ? theme.courseIds[courseIndex + 1] : undefined;

  return (
    <main className="course-page page-wrap">
      <BackButton onClick={onBack} label="Tous les cours" />
      <div className="course-header">
        <div>
          <p className="eyebrow">
            {course.level} <span className="slash">/</span> {course.duration}
          </p>
          <h1>{course.title}</h1>
          <p>{course.description}</p>
        </div>
        <div className="objective-box">
          <span className="eyebrow">Objectifs</span>
          {course.objectives.map((objective) => (
            <span key={objective}>
              <Check size={15} /> {objective}
            </span>
          ))}
        </div>
      </div>
      <div className="course-layout">
        <aside className="section-nav">
          <p className="eyebrow">Dans ce cours</p>
          {course.sections.map((item, index) => (
            <button
              key={item.id}
              className={index === sectionIndex ? "selected" : ""}
              onClick={() => setSectionIndex(index)}
            >
              <span>0{index + 1}</span>
              {item.title}
              <ChevronDown size={15} />
            </button>
          ))}
          <div className="progress-wrap">
            <div>
              <span>Progression</span>
              <strong>
                {Math.round(progress)}%
              </strong>
            </div>
            <div className="progress-bar">
              <span
                style={{width: `${progress}%`}}
              />
            </div>
          </div>
        </aside>
        <article className="lesson">
          <div className="lesson-heading">
            <p className="eyebrow">{section.kicker}</p>
            <h2>{section.title}</h2>
          </div>
          {section.blocks.map((block, index) => (
            <BlockRenderer key={`${section.id}-${index}`} block={block} />
          ))}
          <div className="lesson-nav">
            <button
              onClick={previousSection}
              disabled={sectionIndex === 0 && !previousCourse}
            >
              <ArrowLeft size={16} />
              {sectionIndex === 0
                ? previousCourse
                  ? "Cours précédent"
                  : "Premier cours"
                : "Précédent"}
            </button>
            <button
              className="primary-button"
              onClick={nextSection}
              disabled={sectionIndex === course.sections.length - 1 && !nextCourse}
            >
              {sectionIndex === course.sections.length - 1
                ? nextCourse
                  ? "Cours suivant"
                  : "Cours terminé"
                : "Section suivante"} <ArrowRight size={16} />
            </button>
          </div>
        </article>
      </div>
    </main>
  );
}