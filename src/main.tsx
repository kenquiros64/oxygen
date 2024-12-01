import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import {ThemeProvider} from "./themes/ThemeProvider.tsx";
import CssBaseline from "@mui/material/CssBaseline";


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
      <ThemeProvider>
        <CssBaseline />
        <App />
      </ThemeProvider>
  </React.StrictMode>,
);
