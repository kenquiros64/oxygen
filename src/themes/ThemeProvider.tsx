import React, { createContext, useContext, useEffect, useState, ReactNode } from "react";
import { ThemeProvider as MuiThemeProvider } from "@mui/material/styles";
import { lightTheme, darkTheme } from "./Themes.tsx";
import {LazyStore} from "@tauri-apps/plugin-store";

type Theme = "light" | "dark";

interface ThemeContextProps {
    theme: Theme;
    toggleTheme: () => void;
    setTheme: (theme: Theme) => void;
}

// Create the context
const ThemeContext = createContext<ThemeContextProps | undefined>(undefined);

// Create the store instance
const store = new LazyStore(".settings.dat");

export const ThemeProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
    const [theme, setThemeState] = useState<Theme>("light");

    // Helper to save theme to the Tauri store
    const saveTheme = async (theme: Theme) => {
        try {
            await store.set("theme", theme);
            await store.save();
        } catch (error) {
            console.error("Failed to save theme:", error);
        }
    };

    // Helper to load theme from the Tauri store
    const loadTheme = async () => {
        try {
            const storedTheme = await store.get("theme");
            if (storedTheme) {
                setThemeState(storedTheme as Theme);
            }
        } catch (error) {
            console.error("Failed to load theme:", error);
        }
    };

    // Update theme state and save to store
    const setTheme = (newTheme: Theme) => {
        setThemeState(newTheme);
        saveTheme(newTheme);
    };

    // Toggle theme between light and dark
    const toggleTheme = () => {
        const newTheme = theme === "light" ? "dark" : "light";
        setTheme(newTheme);
    };

    // Load theme on initial mount
    useEffect(() => {
        loadTheme();
    }, []);

    return (
        <ThemeContext.Provider value={{ theme, toggleTheme, setTheme }}>
            <MuiThemeProvider theme={theme === "light" ? lightTheme : darkTheme}>
                {children}
            </MuiThemeProvider>
        </ThemeContext.Provider>
    );
};

// Custom hook for consuming the context
export const useTheme = (): ThemeContextProps => {
    const context = useContext(ThemeContext);
    if (!context) {
        throw new Error("useTheme must be used within a ThemeProvider");
    }
    return context;
};