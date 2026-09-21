import { useNavigate } from "react-router-dom";
import { BookOpen, Network, ArrowRight } from "lucide-react";
import "./Home.css";

export function Home() {
  const navigate = useNavigate();

  return (
    <main className="home-page">
      <section className="home-hero page-wrap">
        <p className="eyebrow">
          <span className="pulse" /> Laboratoire de réseau
        </p>
        <h1>
          Apprendre les réseaux
          <br />
          en <em>pratiquant</em>.
        </h1>
        <p className="home-lead">
          Des cours interactifs pour comprendre la théorie, un playground
          pour construire et simuler vos propres topologies réseau.
        </p>
      </section>

      <section className="home-cards page-wrap">
        <button className="home-card card-courses" onClick={() => navigate("/courses")}>
          <div className="card-icon">
            <BookOpen size={28} />
          </div>
          <div className="card-body">
            <h2>Cours</h2>
            <p>
              Parcourez les cours interactifs : couches réseau, protocoles,
              encapsulation, routage et plus encore.
            </p>
          </div>
          <span className="card-action">
            Explorer les cours <ArrowRight size={15} />
          </span>
        </button>

        <button className="home-card card-netsim" onClick={() => navigate("/netsim")}>
          <div className="card-icon">
            <Network size={28} />
          </div>
          <div className="card-body">
            <h2>Playground</h2>
            <p>
              Créez des machines, configurez des interfaces, reliez-les
              et lancez une simulation pas-à-pas.
            </p>
          </div>
          <span className="card-action">
            Ouvrir le playground <ArrowRight size={15} />
          </span>
        </button>
      </section>
    </main>
  );
}
