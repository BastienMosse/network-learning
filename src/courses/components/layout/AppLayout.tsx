import { Outlet } from "react-router-dom";
import "./AppLayout.css";

import { useTheme } from "../../contexts/ThemeContext";
import Header from "./Header";
import Footer from "./Footer";

export default function AppLayout() {
  const { dark } = useTheme();

  return (
    <div className={dark ? "app dark" : "app"}>
      <Header />
      <Outlet />
      <Footer />
    </div>
  );
}