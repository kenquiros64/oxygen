import { createTheme } from "@mui/material/styles";

const lightTheme = createTheme({
    palette: {
        mode: 'light',
        primary: {
            main: '#2196f3',
            light: '#2ea6d5',
            dark: '#1565c0',
        },
        secondary: {
            main: '#8bc34a',
        },
    },
});

const darkTheme = createTheme({
  palette: {
    mode: "dark",
    primary: {
        main: '#2196f3',
        light: '#2ea6d5',
        dark: '#1565c0',
    },
    secondary: {
        main: '#8bc34a',
    },
  },
});

export { lightTheme, darkTheme };