import { useNavigate, useParams } from "react-router-dom";
import "./Theme.css";

import { useManifest } from "../../contexts/ManifestContext";
import { BackButton } from "../../components/navigation/BackButton";
import { CourseList } from "../../components/courses/CourseList";

export function Theme() {
  const { themeId } = useParams();
  const navigate = useNavigate();
  const manifest = useManifest();

  const onBack = () => { navigate('/') };
  const onCourse = (themeId: string, courseId: string) => { navigate(`/theme/${themeId}/course/${courseId}`); };

  const courses = manifest.courses.filter(
    (course) => course.themeId === themeId,
  );

  const theme = manifest.themes.find(
    (theme) => theme.id === themeId,
  )!;

  return (
    <main className="page-wrap inner-page">
      <BackButton onClick={onBack} />
      <div className="inner-hero">
        <p className="eyebrow">{theme.eyebrow}</p>
        <h1>{theme.title}</h1>
        <p>{theme.description}</p>
      </div>
      <div className="section-heading">
        <div>
          <p className="eyebrow">Cours du thème</p>
          <h2>À votre rythme</h2>
        </div>
        <span className="section-count">
          {courses.length.toString().padStart(2, "0")} cours
        </span>
      </div>
      {courses.length ? (
        <CourseList courses={courses} onCourse={onCourse} />
      ) : (
        <div className="empty-state">
          Les premiers cours de ce parcours sont en préparation.
        </div>
      )}
    </main>
  );
}