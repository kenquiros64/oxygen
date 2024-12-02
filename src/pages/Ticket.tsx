import React, { useState }  from "react";
import {useTicketStore} from "../store/TicketStore.ts";
import {Box, CardContent, Grid2, TextField, Typography} from "@mui/material";
import CssBaseline from "@mui/material/CssBaseline";
import Divider from "@mui/material/Divider";
import IconButton from "@mui/material/IconButton";
import {QuestionMarkOutlined} from "@mui/icons-material";
import { Card, CardOverflow }  from '@mui/joy';


const Ticket: React.FC = () => {

    const {
        selectedRoute,
        selectedStop,
        selectedTime,
        selectedTimetable,
        setSelectedRoute,
        setSelectedStop,
        setSelectedTime,
        setSelectedTimetable,
        incrementCount,
        getRouteStopTimeCount,
        getTotalCountForRouteTime,
      } = useTicketStore();

      // Local state for inputs
      const [code, setCode] = useState("");
      // const [error, setError] = useState({code: ""});
      const [routeInput, setRouteInput] = useState("");
      const [stopInput, setStopInput] = useState("");
      const [timeInput, setTimeInput] = useState("");

      // Handler functions
      const handleSelectRoute = () => {
        setSelectedRoute(routeInput);
        setRouteInput("");
      };

      const handleSelectStop = () => {
        setSelectedStop(stopInput);
        setStopInput("");
      };

      const handleSelectTime = () => {
        setSelectedTime(timeInput);
        setTimeInput("");
      };

      return (
          <Grid2 container sx={{ height: '100vh', margin: 0 }}>
              <CssBaseline />
              <Grid2 size={{ xs: 4 }}
                     sx={{ display: 'flex', flexDirection: 'column', height: '100%', margin: 1
              }}>
                  <Box sx={{ display: 'flex', alignItems: 'center' }}>
                      <TextField
                          label="Code"
                          type="text"
                          variant="outlined"
                          value={code}
                          onChange={(e) => {
                              setCode(e.target.value);
                              // setError((prev) => ({ ...prev, password: "" }));
                          }}
                          // error={error.password !== ""}
                          // helperText={error.password ? "La contraseña es requerida" : ""}
                          style={{ flexGrow: 1 }}
                      />
                      <IconButton aria-label="delete" disabled color="primary">
                          <QuestionMarkOutlined />
                      </IconButton>
                  </Box>
                  <Card sx={{ borderRadius: 0, width: 300, maxWidth: '100%' }}>
                      <CardContent>
                          <Typography >IN DESIGN</Typography>
                          <Typography >AFSL Web App</Typography>
                      </CardContent>
                      <CardOverflow
                          sx={{
                              display: 'flex',
                              flexDirection: 'row',
                              gap: 1,
                              justifyContent: 'space-around',
                              py: 1,
                              borderTop: '1px solid',
                              borderColor: 'divider',
                          }}
                      >
                          <Typography>
                              13
                          </Typography>
                          <Divider orientation="vertical" />
                          <Typography >
                              9
                          </Typography>
                          <Divider orientation="vertical" />
                          <Typography >
                              32
                          </Typography>
                      </CardOverflow>
                  </Card>
              </Grid2>
              <Divider sx={{ height: '100%' }} orientation={"vertical"} flexItem />
              <Grid2 size={{ xs: 4 }} style={{ height: '100%' }}>
                  2
              </Grid2>
              <Divider sx={{ height: '100%' }} orientation={"vertical"} flexItem />
              <Grid2 size={{ xs: 4 }} style={{ height: '100%' }}>
                  3
              </Grid2>
          </Grid2>
      )
};

export default Ticket;