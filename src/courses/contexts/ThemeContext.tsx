import { createContext, useContext } from "react";

export const ThemeContext = createContext<{
  dark: boolean;
  toggleTheme: () => void;
} | null>(null);

export const useTheme = () => {
  const theme = useContext(ThemeContext);

  if (!theme) {
    throw new Error("ThemeProvider missing");
  }

  return theme;
};