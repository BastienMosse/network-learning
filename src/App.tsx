import { useEffect, useId, useState } from "react";
import {
  ArrowLeft,
  ArrowRight,
  ArrowUpRight as ArrowUpRightIcon,
  BookOpen,
  Check,
  ChevronDown,
  Clock3,
  Layers3,
  Moon,
  Play,
  RotateCcw,
  Sun,
  TerminalSquare,
} from "lucide-react";
import { parse } from "yaml";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import "./App.css";
import type {
  ContentBlock,
  Course,
  CourseManifest,
  CourseSummary,
  DemoBlock,
  MermaidBlock,
  Theme,
  FrameBlock,
  FrameField
} from "./types";

const fallbackManifest: CourseManifest = {
  themes: [],
  courses: [],
  courseFiles: {},
};

async function loadManifest() {
  const response = await fetch("/courses/manifest.json");
  return response.ok
    ? ((await response.json()) as CourseManifest)
    : fallbackManifest;
}

async function loadCourse(url: string) {
  const response = await fetch(url);
  if (!response.ok) throw new Error("Impossible de charger ce cours");
  return parse(await response.text()) as Course;
}

function App() {
  const [manifest, setManifest] = useState<CourseManifest>(fallbackManifest);
  const [selectedTheme, setSelectedTheme] = useState<Theme | null>(null);
  const [selectedCourse, setSelectedCourse] = useState<Course | null>(null);
  const [loading, setLoading] = useState(true);
  const [dark, setDark] = useState(false);

  useEffect(() => {
    loadManifest()
      .then(setManifest)
      .finally(() => setLoading(false));
  }, []);

  const openCourse = async (summary: CourseSummary) => {
    setLoading(true);
    setSelectedCourse(await loadCourse(manifest.courseFiles[summary.id]));
    setSelectedTheme(null);
    setLoading(false);
    window.scrollTo({ top: 0, behavior: "smooth" });
  };

  const openTheme = (theme: Theme) => {
    setSelectedTheme(theme);
    setSelectedCourse(null);
    window.scrollTo({ top: 0, behavior: "smooth" });
  };

  const goHome = () => {
    setSelectedTheme(null);
    setSelectedCourse(null);
    window.scrollTo({ top: 0, behavior: "smooth" });
  };

  return (
    <div className={dark ? "app dark" : "app"}>
      <header className="topbar">
        <button
          className="brand"
          onClick={goHome}
          aria-label="Retour à l'accueil"
        >
          <span className="brand-mark">
            <span />
            <span />
            <span />
          </span>
          <span>PAQUET</span>
        </button>
        <nav className="topnav" aria-label="Navigation principale">
          <button
            onClick={goHome}
            className={!selectedTheme && !selectedCourse ? "active" : ""}
          >
            Explorer
          </button>
          <span className="nav-rule" />
          <span className="nav-note">Laboratoire de réseau</span>
        </nav>
        <button
          className="icon-button"
          onClick={() => setDark(!dark)}
          aria-label={
            dark ? "Activer le thème clair" : "Activer le thème sombre"
          }
        >
          {dark ? <Sun size={18} /> : <Moon size={18} />}
        </button>
      </header>

      {loading ? (
        <div className="loading">
          Chargement du laboratoire<span>...</span>
        </div>
      ) : selectedCourse ? (
        <CourseView
          key={selectedCourse.id}
          course={selectedCourse}
          previousCourse={manifest.courses[manifest.courses.findIndex((item) => item.id === selectedCourse.id) - 1]}
          nextCourse={manifest.courses[manifest.courses.findIndex((item) => item.id === selectedCourse.id) + 1]}
          onBack={goHome}
          onPreviousCourse={openCourse}
          onNextCourse={openCourse}
        />
      ) : selectedTheme ? (
        <ThemeView
          theme={selectedTheme}
          manifest={manifest}
          onBack={goHome}
          onCourse={openCourse}
        />
      ) : (
        <HomeView
          manifest={manifest}
          onTheme={openTheme}
          onCourse={openCourse}
        />
      )}
      <footer>
        <span>PAQUET / contenu piloté par fichiers</span>
        <span>v 0.1 · pour apprendre en faisant</span>
      </footer>
    </div>
  );
}

function HomeView({
  manifest,
  onTheme,
  onCourse,
}: {
  manifest: CourseManifest;
  onTheme: (theme: Theme) => void;
  onCourse: (course: CourseSummary) => void;
}) {
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
            onClick={() => onCourse(manifest.courses[0])}
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
        <div className="theme-grid">
          {manifest.themes.map((theme) => (
            <ThemeCard
              key={theme.id}
              theme={theme}
              manifest={manifest}
              onClick={() => onTheme(theme)}
            />
          ))}
        </div>
        <div className="featured-heading">
          <p className="eyebrow">À la une</p>
          <h2>Les premiers paquets</h2>
        </div>
        <div className="course-grid">
          {manifest.courses.map((course, index) => (
            <CourseCard
              key={course.id}
              course={course}
              index={index}
              onClick={() => onCourse(course)}
            />
          ))}
        </div>
      </section>
    </main>
  );
}

function ThemeCard({
  theme,
  manifest,
  onClick,
}: {
  theme: Theme;
  manifest: CourseManifest;
  onClick: () => void;
}) {
  const count = manifest.courses.filter(
    (course) => course.themeId === theme.id,
  ).length;
  return (
    <button className={`theme-card ${theme.accent}`} onClick={onClick}>
      <div className="theme-card-top">
        <span>{theme.eyebrow}</span>
        <ArrowUpRightIcon />
      </div>
      <strong>{theme.title}</strong>
      <p>{theme.description}</p>
      <span className="theme-card-bottom">
        {count ? `${count} cours` : "Bientôt"} <ArrowRight size={15} />
      </span>
    </button>
  );
}

function CourseCard({
  course,
  index,
  onClick,
}: {
  course: CourseSummary;
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
        <ArrowUpRightIcon size={18} />
      </span>
    </button>
  );
}

function ThemeView({
  theme,
  manifest,
  onBack,
  onCourse,
}: {
  theme: Theme;
  manifest: CourseManifest;
  onBack: () => void;
  onCourse: (course: CourseSummary) => void;
}) {
  const courses = manifest.courses.filter(
    (course) => course.themeId === theme.id,
  );
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
        <div className="course-grid">
          {courses.map((course, index) => (
            <CourseCard
              key={course.id}
              course={course}
              index={index}
              onClick={() => onCourse(course)}
            />
          ))}
        </div>
      ) : (
        <div className="empty-state">
          Les premiers cours de ce parcours sont en préparation.
        </div>
      )}
    </main>
  );
}

function CourseView({
  course,
  previousCourse,
  nextCourse,
  onBack,
  onPreviousCourse,
  onNextCourse,
}: {
  course: Course;
  previousCourse?: CourseSummary;
  nextCourse?: CourseSummary;
  onBack: () => void;
  onPreviousCourse: (course: CourseSummary) => void;
  onNextCourse: (course: CourseSummary) => void;
}) {
  const [sectionIndex, setSectionIndex] = useState(0);
  const section = course.sections[sectionIndex];
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
                {Math.round(
                  ((sectionIndex + 1) / course.sections.length) * 100,
                )}
                %
              </strong>
            </div>
            <div className="progress-bar">
              <span
                style={{
                  width: `${((sectionIndex + 1) / course.sections.length) * 100}%`,
                }}
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
              onClick={() => {
                if (sectionIndex > 0) {
                  setSectionIndex(sectionIndex - 1)
                } else if (previousCourse) {
                  onPreviousCourse(previousCourse)
                }
              }}
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
              onClick={() => {
                if (sectionIndex < course.sections.length - 1) {
                  setSectionIndex(sectionIndex + 1)
                } else if (nextCourse) {
                  onNextCourse(nextCourse)
                }
              }}
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

function BackButton({
  onClick,
  label = "Retour à l’exploration",
}: {
  onClick: () => void;
  label?: string;
}) {
  return (
    <button className="back-button" onClick={onClick}>
      <ArrowLeft size={15} /> {label}
    </button>
  );
}

function RichText({ content }: { content: string }) {
  return <div className="rich-text"><ReactMarkdown remarkPlugins={[remarkGfm]}>{content}</ReactMarkdown></div>;
}

type FrameFiller = { kind: "filler"; bits: number };
type FrameSegment = {
  kind: "field";
  field: FrameField;
  bits: number;
  offset: number;
  fieldBits: number; // taille totale réservée (= minimum pour un champ variable)
  isFirst: boolean;
};
type FrameCell = FrameFiller | FrameSegment;

function layoutFrameRows(fields: FrameField[], bitsPerRow: number): FrameCell[][] {
  const rows: FrameCell[][] = [];
  let current: FrameCell[] = [];
  let used = 0;

  const flushRow = () => {
    if (!current.length) return;

    const leftover = bitsPerRow - used;
    if (leftover > 0) {
      const variableCells = current.filter(
        (cell): cell is FrameSegment =>
          cell.kind === "field" && !!cell.field.variable,
      );
      if (variableCells.length > 0) {
        // l'espace libre restant est réparti entre les champs variables de la ligne
        let assigned = 0;
        variableCells.forEach((cell, index) => {
          const isLastVar = index === variableCells.length - 1;
          const share = isLastVar
            ? leftover - assigned
            : Math.floor(leftover / variableCells.length);
          cell.bits += share;
          assigned += share;
        });
      } else {
        current.push({ kind: "filler", bits: leftover });
      }
    }

    rows.push(current);
    current = [];
    used = 0;
  };

  for (const field of fields) {
    // pour un champ variable, "bits" = taille MINIMALE réservée (0 par défaut)
    let remaining = field.variable ? (field.bits ?? 0) : field.bits;
    let offset = 0;

    // un champ variable sans minimum doit quand même exister dans la ligne
    // pour pouvoir récupérer l'espace libre au moment du flush
    if (field.variable && remaining === 0) {
      current.push({
        kind: "field",
        field,
        bits: 0,
        offset: 0,
        fieldBits: 0,
        isFirst: true,
      });
      continue;
    }

    while (remaining > 0) {
      const available = bitsPerRow - used;
      const take = Math.min(available, remaining);

      current.push({
        kind: "field",
        field,
        bits: take,
        offset,
        fieldBits: field.variable ? field.bits ?? 0 : field.bits,
        isFirst: offset === 0,
      });

      used += take;
      offset += take;
      remaining -= take;

      if (used >= bitsPerRow) flushRow();
    }
  }

  flushRow();
  return rows;
}

function FrameDiagram({ block }: { block: FrameBlock }) {
  const bitsPerRow = block.bitsPerRow ?? 32;
  const rows = layoutFrameRows(block.fields, bitsPerRow);

  return (
    <figure className="frame-block">
      {block.title && <figcaption>{block.title}</figcaption>}
      <div
        className="frame-ruler"
        style={{ gridTemplateColumns: `repeat(${bitsPerRow}, 1fr)` }}
      >
        {Array.from({ length: bitsPerRow }, (_, i) => (
          <span key={i}>{i % 8 === 0 ? i : ""}</span>
        ))}
      </div>
      {rows.map((row, rowIndex) => (
        <div className="frame-row" key={rowIndex}>
          {row.map((item, itemIndex) => {
            if (item.kind === "filler") {
              return (
                <div
                  key={itemIndex}
                  className="frame-cell frame-filler"
                  style={{ flexBasis: `${(item.bits / bitsPerRow) * 100}%` }}
                />
              );
            }

            const { field, bits, offset, fieldBits, isFirst } = item;
            const isVariable = !!field.variable;

            return (
              <div
                key={`${field.id}-${offset}`}
                className={`frame-cell ${field.color || "blue"}${
                  isVariable ? " frame-variable" : ""
                }${!isFirst ? " frame-continued" : ""}`}
                style={{
                  flexBasis: `${(bits / bitsPerRow) * 100}%`,
                  ...(isVariable
                    ? {
                        backgroundImage:
                          "repeating-linear-gradient(45deg, rgba(0,0,0,0.06) 0 8px, transparent 8px 16px)",
                      }
                    : {}),
                }}
                title={field.description}
              >
                <strong>
                  {field.label}
                  {!isFirst && " (suite)"}
                </strong>
                <small>
                  {isVariable
                    ? "longueur variable"
                    : fieldBits <= bitsPerRow
                      ? `${fieldBits} bit${fieldBits > 1 ? "s" : ""}`
                      : `${offset}-${offset + bits - 1} / ${fieldBits} bits`}
                </small>
              </div>
            );
          })}
        </div>
      ))}
    </figure>
  );
}

function BlockRenderer({ block }: { block: ContentBlock }) {
  if (block.type === "text")
    return (
      <section className="text-block">
        {block.title && <h3>{block.title}</h3>}
        <RichText content={block.content} />
      </section>
    );
  if (block.type === "info" || block.type === "warning")
    return (
      <aside className={`callout ${block.type}`}>
        <span className="callout-tag">
          {block.type === "info" ? "À retenir" : "Point de vigilance"}
        </span>
        <strong>{block.title}</strong>
        <p>{block.content}</p>
      </aside>
    );
  if (block.type === "code")
    return (
      <div className="code-block">
        <div>
          <TerminalSquare size={16} />
          <span>{block.language || "exemple"}</span>
        </div>
        <pre>{block.code}</pre>
      </div>
    );
  if (block.type === "mermaid") return <MermaidDiagram block={block} />;
  if (block.type === "image")
    return (
      <figure className="image-block">
        {block.title && <h3>{block.title}</h3>}
        <img src={block.src} alt={block.alt || block.title || "Illustration du cours"} />
        <figcaption>{block.caption}</figcaption>
      </figure>
    );
  if (block.type === "demo") return <Demo block={block} />;
  if (block.type === "frame") return <FrameDiagram block={block} />;
  return null;
}

function MermaidDiagram({ block }: { block: MermaidBlock }) {
  const diagramId = `mermaid-${useId().replace(/:/g, "")}`;
  const [svg, setSvg] = useState("");
  const [error, setError] = useState("");

  useEffect(() => {
    let cancelled = false;
    import("mermaid").then(({ default: mermaid }) => {
      mermaid.initialize({ startOnLoad: false, securityLevel: "strict", theme: "base", flowchart: { useMaxWidth: true } });
      return mermaid.parse(block.diagram).then(() => mermaid.render(diagramId, block.diagram));
    }).then(({ svg: renderedSvg }) => {
      if (!cancelled) {
        setSvg(renderedSvg);
        setError("");
      }
    }).catch(() => {
      if (!cancelled) {
        setSvg("");
        setError("Ce diagramme Mermaid contient une erreur de syntaxe.");
      }
    });
    return () => { cancelled = true; };
  }, [block.diagram, diagramId]);

  return (
    <figure className="mermaid-block">
      {block.title && <figcaption>{block.title}</figcaption>}
      {error ? <p className="mermaid-error">{error}</p> : svg ? <div className="mermaid-diagram" style={{ width: block.width || "100%", ...(block.height ? { height: block.height } : {}) }} dangerouslySetInnerHTML={{ __html: svg }} /> : <p className="mermaid-loading">Génération du diagramme...</p>}
    </figure>
  );
}

function Demo({ block }: { block: DemoBlock }) {
  const [stepIndex, setStepIndex] = useState(0);
  const [playing, setPlaying] = useState(false);
  const step = block.steps[stepIndex];
  const nodes = block.nodes ?? [];
  const imageMode = nodes.length === 0;
  const nodePosition = (nodeId: string) => {
    const nodeIndex = nodes.findIndex((node) => node.id === nodeId);
    if (nodeIndex < 0) return null;
    const configuredPosition = nodes[nodeIndex].position;
    return configuredPosition || {
      x: ((nodeIndex * 2 + 1) / (nodes.length * 2)) * 100,
      y: 50,
    };
  };
  const packets = step.packets || (step.packet ? [step.packet] : []);
  const packetGroups = step.packetGroups || (step.sequential ? packets.map((packet) => [packet]) : [packets]);
  const packetAnimations = packetGroups.flatMap((group, groupIndex) => group.map((packet) => ({
    packet,
    delay: groupIndex * 950,
    intermediate: groupIndex < packetGroups.length - 1,
  })));
  const connections = block.connections || nodes.slice(0, -1).map((node, index) => ({
    from: node.id,
    to: nodes[index + 1].id,
  }));
  useEffect(() => {
    if (!playing) return;
    const timer = window.setTimeout(() => {
      if (stepIndex >= block.steps.length - 1) setPlaying(false);
      else setStepIndex(stepIndex + 1);
    }, 1700);
    return () => window.clearTimeout(timer);
  }, [playing, stepIndex, block.steps.length]);
  return (
    <section className="demo-block">
      <div className="demo-title">
        <div>
          <span className="demo-kicker">
            <span className="live-dot" /> Démo interactive
          </span>
          <h3>{block.title}</h3>
          <p>{block.intro}</p>
        </div>
        <button
          className="reset-button"
          onClick={() => {
            setStepIndex(0);
            setPlaying(false);
          }}
        >
          <RotateCcw size={15} /> Réinitialiser
        </button>
      </div>
      <div className="demo-stage">
        {imageMode ? (
          <div className="demo-image-stage">
            {step.media ? (
              <figure className="demo-image-figure">
                <img src={step.media.src} alt={step.media.alt} />
                {step.media.caption && <figcaption>{step.media.caption}</figcaption>}
              </figure>
            ) : (
              <span className="demo-image-empty">Aucune image pour cette étape</span>
            )}
          </div>
        ) : (
        <div className="demo-network">
          {block.segments?.map((segment) => (
            <div
              key={segment.id}
              className={`demo-segment ${segment.color || "blue"}`}
              style={{
                left: `${segment.x}%`,
                top: `${segment.y}%`,
                width: `${segment.width}%`,
                height: `${segment.height}%`,
              }}
            >
              <span>{segment.label}</span>
            </div>
          ))}
          {nodes.map((node) => (
            <div
              key={node.id}
              className={`demo-node ${node.kind} ${step.active?.includes(node.id) ? "active" : ""}`}
              style={
                {
                  left: `${nodePosition(node.id)?.x ?? 50}%`,
                  top: `${nodePosition(node.id)?.y ?? 50}%`,
                } as React.CSSProperties
              }
            >
              <span className="node-symbol">
                {node.kind === "service" ? "◆" : "□"}
              </span>
              <strong>{node.label}</strong>
              <small>
                {node.kind === "service" ? "service" : "équipement"}
              </small>
            </div>
          ))}
          <svg className="demo-connections" viewBox="0 0 100 100" preserveAspectRatio="none" aria-hidden="true">
            {connections.map((connection) => {
              const from = nodePosition(connection.from);
              const to = nodePosition(connection.to);
              if (!from || !to) return null;
              return <line key={`${connection.from}-${connection.to}`} x1={from.x} y1={from.y} x2={to.x} y2={to.y} />;
            })}
          </svg>
          {packetAnimations.map(({ packet, delay, intermediate }, packetIndex) => {
            const packetFrom = nodePosition(packet.from);
            const packetTo = nodePosition(packet.to);
            if (!packetFrom || !packetTo) return null;
            return (
              <div
                key={`${stepIndex}-${packetIndex}-${packet.from}-${packet.to}`}
                className={`moving-packet ${intermediate ? "packet-intermediate" : ""}`}
                style={
                  {
                    "--packet-from-x": `${packetFrom.x}%`,
                    "--packet-from-y": `${packetFrom.y}%`,
                    "--packet-to-x": `${packetTo.x}%`,
                    "--packet-to-y": `${packetTo.y}%`,
                    "--packet-delay": `${delay}ms`,
                  } as React.CSSProperties
                }
              >
                <span />
              </div>
            );
          })}
        </div>
        )}
        <div className="demo-panel">
          <span className="panel-label">État actuel</span>
          <strong>{step.panel}</strong>
          <p>{step.description}</p>
          {!imageMode && step.media && (
            <figure className="demo-media">
              <img src={step.media.src} alt={step.media.alt} />
              {step.media.caption && <figcaption>{step.media.caption}</figcaption>}
            </figure>
          )}
          {step.table && (
            <div className="demo-table-wrap">
              <table className="demo-table">
                <thead>
                  <tr>
                    {step.table.headers.map((header) => <th key={header}>{header}</th>)}
                  </tr>
                </thead>
                <tbody>
                  {step.table.rows.map((row, rowIndex) => (
                    <tr key={`row-${rowIndex}`}>
                      {row.map((cell, cellIndex) => <td key={`cell-${rowIndex}-${cellIndex}`}>{cell}</td>)}
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
          <span className="panel-step">
            0{stepIndex + 1} <i>/</i> 0{block.steps.length}
          </span>
        </div>
      </div>
      <div className="demo-controls">
        <div className="step-dots">
          {block.steps.map((item, index) => (
            <button
              key={item.title}
              className={
                index === stepIndex
                  ? "current"
                  : index < stepIndex
                    ? "done"
                    : ""
              }
              onClick={() => {
                setStepIndex(index);
                setPlaying(false);
              }}
              aria-label={`Aller à l'étape ${index + 1}`}
            >
              <span>{index < stepIndex ? <Check size={11} /> : index + 1}</span>
            </button>
          ))}
        </div>
        <div className="demo-actions">
          <button
            onClick={() => setStepIndex(Math.max(0, stepIndex - 1))}
            disabled={stepIndex === 0}
          >
            <ArrowLeft size={14} />
          </button>
          <button className="play-button" onClick={() => setPlaying(!playing)}>
            {playing ? (
              "Pause"
            ) : (
              <>
                <Play size={13} fill="currentColor" /> Lire
              </>
            )}
          </button>
          <button
            onClick={() =>
              setStepIndex(Math.min(block.steps.length - 1, stepIndex + 1))
            }
            disabled={stepIndex === block.steps.length - 1}
          >
            <ArrowRight size={14} />
          </button>
        </div>
      </div>
    </section>
  );
}

export default App;
