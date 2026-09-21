import { useLocation, useNavigate } from "react-router-dom";
import { Moon, Sun } from "lucide-react";
import "./Header.css";

import { useTheme } from "../../contexts/ThemeContext";

export default function Header() {
  const navigate = useNavigate();
  const location = useLocation();

  const { dark, toggleTheme } = useTheme();

  const isCourses = location.pathname.startsWith("/courses");
  const isNetsim = location.pathname.startsWith("/netsim");

  const goHome = () => { navigate("/"); };

  const themeLabel = dark
    ? "Activer le thème clair"
    : "Activer le thème sombre";

  const icone = dark
    ? (<Sun size={18} />)
    : (<Moon size={18} />);

  return (
    <header className="topbar">
      <button className="brand" onClick={goHome} aria-label="Retour à l'accueil" >
        <span className="brand-mark">
          <span />
          <span />
          <span />
        </span>
        <span>PAQUET</span>
      </button>

      <nav className="topnav" aria-label="Navigation principale" >
        <button onClick={() => navigate("/courses")} className={isCourses ? "active" : ""}>
          Cours
        </button>

        <span className="nav-rule" />

        <button onClick={() => navigate("/netsim")} className={isNetsim ? "active" : ""}>
          Playground
        </button>
      </nav>

      <button className="icon-button" onClick={toggleTheme} aria-label={themeLabel}>
        {icone}
      </button>
    </header>
  );
}
