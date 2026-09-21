import { createBrowserRouter } from "react-router-dom";

import AppLayout from "../courses/components/layout/AppLayout";

import { Home } from "../pages/Home/Home";
import { Explorer } from "../pages/Explorer/Explorer";
import { Theme } from "../pages/Theme/Theme";
import { Course } from "../pages/Course/Course";
import { Playground } from "../pages/Playground/Playground";
import { Download } from "../pages/Download/Download";

const basename = import.meta.env.BASE_URL.replace(/\/+$/, "") || undefined;

export const router = createBrowserRouter(
  [
    { path: "/download", element: <Download /> },
    {
      element: <AppLayout />,
      children: [
        { path: "/", element: <Home /> },
        { path: "/courses", element: <Explorer /> },
        { path: "/courses/theme/:themeId", element: <Theme /> },
        { path: "/courses/theme/:themeId/course/:courseId", element: <Course /> },
        { path: "/netsim", element: <Playground /> },
      ],
    },
  ],
  { basename },
);
