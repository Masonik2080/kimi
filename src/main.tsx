import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { TrayMenu } from "./components/TrayMenu";
import "./index.css";

// Определяем какой компонент рендерить по хэшу URL
const isTrayMenu = window.location.hash === "#tray";

// Добавляем класс для прозрачного фона tray меню
if (isTrayMenu) {
  document.documentElement.classList.add("tray-menu");
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    {isTrayMenu ? <TrayMenu /> : <App />}
  </React.StrictMode>,
);
