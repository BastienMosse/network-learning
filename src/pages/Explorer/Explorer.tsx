import { ArrowRight } from 'lucide-react'
import "./Explorer.css";

import { useNavigate } from "react-router-dom";
import { useManifest } from "../../courses/contexts/ManifestContext";
import { ThemeList } from '../../courses/components/themes/ThemeList';
import { CourseList } from '../../courses/components/courses/CourseList';

export function Explorer() {
  const navigate = useNavigate();
  const manifest = useManifest();

  const onTheme = (themeId: string) => { navigate(`/courses/theme/${themeId}`); };
  const onCourse = (themeId: string, courseId: string) => { navigate(`/courses/theme/${themeId}/course/${courseId}`); };

  const firstCourse = manifest.courses[0];
  const firstCourseTheme = firstCourse 
    ? manifest.themes.find((theme) => theme.courseIds.includes(firstCourse.id) )
    : undefined;

  return (
    <main>
      <section className="hero-section page-wrap">
        <div className="hero-copy">
          <p className="eyebrow">
            <span className="pulse" /> Cours interactifs · réseaux informatiques
          </p>
          <h1>
            Comprendre ce qui
            <br />
            <em>circule</em> entre les machines.
          </h1>
          <p className="hero-lead">
            Un espace pour apprendre les réseaux en manipulant des paquets, des
            couches et des idées. Les cours sont écrits en fichiers, l'interface
            s'occupe du reste.
          </p>
          <button
            className="primary-button"
            onClick={() => onCourse(firstCourseTheme!.id, firstCourse.id)}
          >
            Commencer le parcours <ArrowRight size={16} />
          </button>
        </div>
        <div className="hero-visual" aria-hidden="true">
          <div className="orbit orbit-one" />
          <div className="orbit orbit-two" />
          <div className="network-node node-a">PC</div>
          <div className="network-node node-b">SW</div>
          <div className="network-node node-c">R</div>
          <div className="network-node node-d">WEB</div>
          <div className="network-line line-a" />
          <div className="network-line line-b" />
          <div className="network-line line-c" />
          <div className="packet-dot" />
          <span className="visual-label label-a">source</span>
          <span className="visual-label label-b">route</span>
          <span className="visual-label label-c">destination</span>
        </div>
      </section>
      <section className="catalog page-wrap">
        <div className="section-heading">
          <div>
            <p className="eyebrow">01 / Parcours</p>
            <h2>Choisir un point de départ</h2>
          </div>
          <span className="section-count">
            {manifest.courses.length.toString().padStart(2, "0")} cours
            disponibles
          </span>
        </div>
        <ThemeList manifest={manifest} onTheme={onTheme} />
        <div className="featured-heading">
          <p className="eyebrow">À la une</p>
          <h2>Les premiers paquets</h2>
        </div>
        <CourseList courses={manifest.courses} onCourse={onCourse} />
      </section>
    </main>
  );
}