import React, { useState }  from "react";
import {TextField, Box, Typography, Grid2, Paper} from "@mui/material";
import {LoadingButton} from "@mui/lab";
import { useNavigate } from "react-router";

import backgroundLogo from "../assets/background.png";
import logo from "../assets/white_logo.png";

import {useTheme} from "../themes/ThemeProvider.tsx";
import {ThemeSwitch} from "../components/ThemeSwitch.tsx";
import CssBaseline from "@mui/material/CssBaseline";
import {ArrowForward} from "@mui/icons-material";
import {useAuthState} from "../states/AuthState.ts";
import { ErrorResponse } from "../models/models.ts";


const Login: React.FC = () => {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [inputError, setInputError] = useState({username: "", password: ""});
    const [loading, setLoading] = useState(false);
    const { login } = useAuthState();
    const navigate = useNavigate();

    const { theme, toggleTheme } = useTheme();

    const handleLogin = async () => {
        if (!username) {
            setInputError({username: "Username es requerido", password: ""});
            return;
        }
        if (!password) {
            setInputError({username: "", password: "Password es requerido"});
            return;
        }
        
        setLoading(true);
        login(username, password).then(() => {
            navigate("/home"); // Navigate to HomeLayout
        }).catch((error: ErrorResponse) => {
            console.log("Error: ", error);
            if (error?.code === 1) {
                setInputError({username: error.error, password: ""});
            }
            if (error?.code === 2) {
                setInputError({username: "", password: error.error});
            }
        }).finally(() => {
            setLoading(false)
        });

    }

    return (
        <Grid2 container sx={{ height: '100vh', margin: 0 }}>
            <CssBaseline />
            {/* Left Side */}
            <Grid2 size={{ xs: 12, md: 6}} style={{ height: '100%' }}>
                <Box
                    sx={{
                        position: 'relative', // Required to position the text over the image
                        backgroundImage: `url(${backgroundLogo})`,
                        backgroundSize: 'cover',
                        backgroundPosition: 'center',
                        height: '100%',
                        width: '100%',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        textAlign: 'center',
                    }}
                >
                    <Grid2 container style={{ height: '100vh', width: '100%' }}>
                        <Grid2 size={12} rowSpacing={2} >
                            <Paper
                                variant="outlined"
                                sx={{
                                    display: 'flex',
                                    alignItems: 'center',
                                    padding: '16px',
                                    backgroundColor: 'transparent', // Makes it transparent
                                    boxShadow: 'none', // Removes any shadow if needed
                                    border: 'none', // Removes the border if not needed
                                }}
                            >
                                {/* Image on the left */}
                                <Box
                                    component="img"
                                    src={logo}
                                    alt="Logo"
                                    sx={{
                                        width: '150px',
                                        height: '110px',
                                        marginRight: '16px',
                                        objectFit: 'contain',
                                    }}
                                />

                                {/* Texts on the right */}
                                <Box sx={{ textAlign: 'left' }}>
                                    <Typography
                                        variant="h4"
                                        component="div"
                                        fontWeight="bold"
                                        color={'grey.300'}
                                    >
                                        Transportes
                                    </Typography>
                                    <Typography
                                        variant="h6"
                                        component="div"
                                        color={'grey.300'}
                                        fontWeight={200}
                                    >
                                        El Puma Pardo S.A
                                    </Typography>
                                </Box>
                            </Paper>
                        </Grid2>

                        <Grid2 size={12} rowSpacing={2} sx={{
                                position: 'absolute',
                                bottom: 0,
                                right: 0,
                                padding: '8px',
                                textAlign: 'right',
                            }}
                        >
                            <Typography variant="body2"
                                        component={'div'}
                                        color={'grey.300'}
                                        fontWeight={200}
                                        fontSize={14}
                            >
                                oxygen 1.0.0
                            </Typography>
                        </Grid2>
                    </Grid2>
                </Box>
            </Grid2>

            {/* Right Side */}
            <Grid2  size={{ xs: 12, md: 6}} style={{ height: '100%' }}>
                <Box  sx={{ width: "100%" }}>
                    <ThemeSwitch
                        checked={theme === "dark"}
                        onClick={toggleTheme}
                        sx={{
                            position: 'absolute',
                            top: 5,
                            right: 5,
                        }}
                    />
                    <Grid2
                        container
                        sx={{
                            width: '100%',
                            height: '100vh', // Full viewport height
                            display: 'flex',
                            justifyContent: 'center', // Centers horizontally
                            alignItems: 'center', // Centers vertically
                        }}
                    >
                        <Grid2
                            size={{ xs: 12 }}
                            sx={{
                                margin: 10,
                                width: '100%',
                            }}
                        >
                            <Typography variant="h2" textAlign="left" >
                                Bienvenidos
                            </Typography>
                            <Typography variant="subtitle1" textAlign="left" mb={5} color={"primary"} fontWeight={200}>
                                Gestión rápida y sencilla para el transporte de pasajeros
                            </Typography>


                            <Typography variant="h5" textAlign="center" mb={2} fontWeight={200}>
                                Iniciar Sesión
                            </Typography>
                            {/* Fields */}
                            <Box component="form" noValidate>
                                <TextField
                                    label="Username"
                                    variant="outlined"
                                    fullWidth
                                    value={username}
                                    onChange={(e) => {
                                        setUsername(e.target.value);
                                        setInputError((prev) => ({ ...prev, username: "" }));
                                    }}
                                    error={ inputError.username !== "" }
                                    helperText={inputError.username ? inputError.username : ""}
                                    style={{ marginBottom: "2rem" }}
                                />
                                <TextField
                                    label="Password"
                                    type="password"
                                    variant="outlined"
                                    fullWidth
                                    value={password}
                                    onChange={(e) => {
                                        setPassword(e.target.value);
                                        setInputError((prev) => ({ ...prev, password: "" }));
                                    }}
                                    error={inputError.password !== ""}
                                    helperText={inputError.password ? inputError.password : ""}
                                    style={{ marginBottom: "2rem" }}
                                />

                                {/* Login Button */}
                                <LoadingButton
                                    fullWidth
                                    loading={loading}
                                    loadingPosition={"end"}
                                    size={"large"}
                                    variant="contained"
                                    color="secondary"
                                    sx={{ fontSize: 17, fontWeight: '200' }}
                                    onClick={handleLogin}
                                    endIcon={<ArrowForward />}
                                >
                                    Ingresar
                                </LoadingButton>
                            </Box>
                        </Grid2>
                    </Grid2>
                </Box>
            </Grid2>
        </Grid2>
    )
}

export default Login;