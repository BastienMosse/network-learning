import { createBrowserRouter } from "react-router-dom";

import AppLayout from "../components/layout/AppLayout";

import { Home } from "../pages/Home/Home";
import { Theme } from "../pages/Theme/Theme";
import { Course } from "../pages/Course/Course";

export const router = createBrowserRouter([
  {
    element: <AppLayout />,
    children: [
      { path: "/", element: <Home /> },
      { path: "/theme/:themeId", element: <Theme /> },
      { path: "/theme/:themeId/course/:courseId", element: <Course /> },
  ]}
]);