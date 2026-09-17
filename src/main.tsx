import ReactDOM from "react-dom/client";
import React from "react";
import App from "./App";
import "./index.css";
import "./styles/common.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
