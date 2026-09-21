import { createBrowserRouter } from "react-router-dom";

import AppLayout from "../courses/components/layout/AppLayout";

import { Home } from "../pages/Home/Home";
import { Explorer } from "../pages/Explorer/Explorer";
import { Theme } from "../pages/Theme/Theme";
import { Course } from "../pages/Course/Course";
import { Playground } from "../pages/Playground/Playground";

export const router = createBrowserRouter([
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
]);
