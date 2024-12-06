import React from "react";
import {Box, Button, Card, CardContent, CardMedia, FormControl, MenuItem, Select, SelectChangeEvent, Typography} from "@mui/material";
import bus1Icon from "../assets/bus1.png";
import routeIcon from "../assets/routeIcon.png";
import Divider from "@mui/material/Divider";
import {
    AirlineSeatLegroomNormal,
    ArrowDropDown,
    ArrowDropUp,
    DepartureBoard,
    Elderly,
    Money
} from "@mui/icons-material";
import IconButton from "@mui/material/IconButton";
import {useTheme} from "../themes/ThemeProvider.tsx";
import {useTicketState} from "../states/TicketState.ts";

const HomeCard: React.FC = () => {
    const {theme} = useTheme();
    const {
        selectedTime, setSelectedTime,
        selectedRoute,
        selectedStop,
        selectedTimetable
    } = useTicketState();

    const timetable =
        selectedTimetable === 'normal'
            ? selectedRoute?.timetable
            : selectedRoute?.holidayTimetable;

    const handleChange = (event: SelectChangeEvent<string>) => {
        const value = event.target.value as string;
        const [hour, minute] = value.split(':').map(Number);
        setSelectedTime({ hour, minute });
    };

    const handleIncrement = () => {
        const times = selectedTimetable === "normal"
            ? selectedRoute?.timetable
            : selectedRoute?.holidayTimetable;

        if (!times || times.length === 0) {
            console.error("Times not found or empty");
            return;
        }

        const currentIndex = times.findIndex((time) => {
            return time.hour === selectedTime?.hour && time.minute === selectedTime?.minute;
        });

        if (currentIndex === -1) {
            console.warn("Current time not found");
            return;
        }
    
        if (currentIndex < times.length - 1) {
            setSelectedTime(times[currentIndex + 1]);
        } else {
            console.warn("Cannot increment: already at the last time");
        }
    };

    const handleDecrement = () => {
        const times = selectedTimetable === "normal"
            ? selectedRoute?.timetable
            : selectedRoute?.holidayTimetable;

        if (!times || times.length === 0) {
            console.error("Times not found or empty");
            return;
        }
    
        const currentIndex = times.findIndex((time) => {
            return time.hour === selectedTime?.hour && time.minute === selectedTime?.minute;
        });
    
        if (currentIndex === -1) {
            console.warn("Current time not found");
            return;
        }
    
        if (currentIndex > 0) {
            setSelectedTime(times[currentIndex - 1]);
        } else {
            console.warn("Cannot decremment: already at the first time");
        }
    };

    return (
        <Card sx={{ borderRadius: 0, width: '100%', backgroundColor: theme === "light" ? '#FAFAFA' : 'default' }}>
            <CardMedia
                component="img"
                image={bus1Icon}
                title="bus"
                sx={{ width: '50%', margin: 'auto', pt: 2 }}
            />
            <CardContent>
                <Box
                    sx={{
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'space-between',
                        mb: 2,
                    }}
                >
                    {/* Left Side - Image */}
                    <Box sx={{ justifyContent: 'left', width: '35%' }}>
                        <CardMedia
                            component="img"
                            image={routeIcon}
                            title="bus"
                            sx={{ height: "90px", width: '60px', ml: 2 }}
                        />
                    </Box>
                    {/* Right Side - Text */}
                    <Box sx={{ display: 'flex', flexDirection: 'column', justifyContent: 'left', width: '65%' }}>
                        <Typography variant="h5" sx={{ mb: 2, pb: 2, color: "text.secondary", fontWeight: "200" }}>
                            {selectedRoute?.departure}
                        </Typography>
                        <Typography variant="h5" sx={{ color: "text.secondary", fontWeight: "200" }} >
                            {selectedRoute?.destination}
                        </Typography>
                    </Box>
                </Box>
                <Box sx={{ display: 'flex', justifyContent: "center" }}>
                    <Typography variant="h3" color="primary" textAlign="center">
                        {selectedStop?.name}
                    </Typography>
                </Box>
                <Divider sx={{ my: 2 }} />
                <Box sx={{ display: 'flex', justifyContent: "center" }}>
                    <Typography variant="h5" color="text.secondary" sx={{ textAlign:"center" }}>
                        Horario Normal
                    </Typography>
                </Box>
                <Box sx={{ display: 'flex', alignItems:"center"  }}>
                    <FormControl
                        variant="standard"
                        sx={{
                            flexGrow: 1,
                            borderRadius: 1,
                            padding: "8px 12px",
                            "&:hover": {
                                borderColor: "primary.main",
                                cursor: "pointer",
                            },
                            "& .MuiSelect-select": {
                                display: "flex",
                                alignItems: "center",
                            },
                        }}
                    >
                        <Select
                            value={
                                selectedTime
                                    ? `${selectedTime.hour.toString().padStart(2, '0')}:${selectedTime.minute
                                        .toString()
                                        .padStart(2, '0')}`
                                    : ''
                            }
                            onChange={handleChange}
                            displayEmpty
                            renderValue={(time) => (
                                <Box sx={{ display: 'flex', alignItems: 'center' }}>
                                    <DepartureBoard fontSize="large" sx={{ mr: 1, color: 'gray', pb: 0.5 }} />
                                    <Box sx={{ fontWeight: 'bold', fontSize: '2.3rem' }}>
                                        {time || 'Select Time'}
                                    </Box>
                                </Box>
                            )}
                            sx={{
                                fontSize: '2.3rem',
                                fontWeight: 'bold',
                                '& .MuiSelect-icon': { display: 'none' }, // Hide default dropdown icon
                            }}
                        >
                            {timetable?.length ? (
                                timetable.map((time) => (
                                    <MenuItem
                                        key={`${time.hour}:${time.minute}`}
                                        value={`${time.hour}:${time.minute}`}
                                    >
                                        {`${time.hour.toString().padStart(2, '0')}:${time.minute
                                            .toString()
                                            .padStart(2, '0')}`}
                                    </MenuItem>
                                ))
                            ) : (
                                <MenuItem disabled>No times available</MenuItem>
                            )}
                        </Select>
                    </FormControl>
                    <Box sx={{ display: 'flex', alignItems:"center", flexDirection:"column", ml:1 }}   >
                        <IconButton onClick={handleDecrement} size="small" sx={{ paddingTop: "2px" }}>
                            <ArrowDropUp fontSize="large" />
                        </IconButton>
                        <IconButton onClick={handleIncrement} size="small" sx={{ paddingBottom: "2px" }}>
                            <ArrowDropDown fontSize="large" />
                        </IconButton>
                    </Box>
                </Box>
            </CardContent>

            <Box
                sx={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-around',
                    py: 2,
                    borderTop: '1px solid',
                    borderColor: 'divider',
                    backgroundColor: theme === "light" ? '#EEEEEE' : 'default'
                }}
            >
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
                    <AirlineSeatLegroomNormal color="secondary" fontSize={"large"} />
                    <Typography variant="h5" sx={{ fontWeight: '200' }}>13</Typography>
                </Box>
                <Divider orientation="vertical" flexItem />
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
                    <Elderly sx={{ color: "#FFC107" }} fontSize={"large"} />
                    <Typography variant="h5" sx={{ fontWeight: '200' }}>9</Typography>
                </Box>
                <Divider orientation="vertical" flexItem />
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                    <Typography variant="h5" sx={{ fontWeight: '200' }}>Total:</Typography>
                    <Typography variant="h5" sx={{ fontWeight:"bold" }}>32</Typography>
                </Box>
            </Box>
            <Box
                sx={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    borderTop: '1px solid',
                    borderColor: 'divider',
                    width: '100%'
                }}
            >
                <Button
                    fullWidth
                    // loading={loading}
                    size={"large"}
                    variant="contained"
                    startIcon={<Money />}
                    sx={{ fontSize: 20, py: 2, borderRadius: 0, backgroundColor: "#FBC02D" }}
                    // onClick={handleLogin}
                >
                    ₡0
                </Button>
                <Button
                    fullWidth
                    // loading={loading}
                    size={"large"}
                    variant="contained"
                    startIcon={<Money />}
                    color="secondary"
                    sx={{ fontSize: 20, py: 2, borderRadius: 0 }}
                    // onClick={handleLogin}
                >
                    ₡300
                </Button>
            </Box>
        </Card>
    )
}

export default HomeCard;