import React, {useEffect} from "react";
import {useTicketState} from "../states/TicketState.ts";
import {
    Box,
    Grid2,
    TextField,
} from "@mui/material";
import Divider from "@mui/material/Divider";
import IconButton from "@mui/material/IconButton";
import {
    QuestionMarkOutlined,
} from "@mui/icons-material";
import HomeCard from "../components/HomeCard.tsx";
import {useRoutesStore} from "../stores/RoutesStore.ts";


const Ticket: React.FC = () => {
    const { routes, fetchRoutes } = useRoutesStore();
    const {
        setSelectedRoute,
        setSelectedStop,
        setSelectedTimetable
    } = useTicketState();

    useEffect(() => {
        // Fetch routes when the component mounts
        fetchRoutes();
      }, [fetchRoutes]);
    
      useEffect(() => {
        if (routes.length > 0) {
            let stopIndex = routes[0].stops.findIndex((stop) => stop.isMain);

            console.log("ROUTES: " + routes)
            setSelectedRoute(routes[0]);
            setSelectedTimetable('normal');
            setSelectedStop(routes[0].stops[stopIndex]);
        }
      }, [routes, setSelectedRoute, setSelectedTimetable, setSelectedStop]);
    
    const {
        code, setCode,
    } = useTicketState();

      return (
          <Grid2 container sx={{ height: '100%', margin: 0 }}>
              <Grid2 size="grow" sx={{
                  display: 'flex', flexDirection: 'column', height: '100%', padding: 2, gap: 2
              }}>
                  <Box sx={{ display: 'flex', alignItems: 'center' }}>
                      <TextField
                          label="Code"
                          type="text"
                          variant="outlined"
                          value={code ?? ""}
                          onChange={(e) => {
                              setCode(e.target.value);
                              // setError((prev) => ({ ...prev, password: "" }));
                          }}
                          // error={error.password !== ""}
                          // helperText={error.password ? "La contraseña es requerida" : ""}
                          style={{ flexGrow: 1 }}
                      />
                      <IconButton aria-label="faq-code" color="primary">
                          <QuestionMarkOutlined />
                      </IconButton>
                  </Box>
                  <Box sx={{ alignItems: 'center', display: 'flex', height: '100%' }}>
                    <HomeCard />
                  </Box>
              </Grid2>
              <Divider sx={{ height: '100%' }} orientation={"vertical"} flexItem />
              <Grid2 size="grow" sx={{ height: '100%' }}>
                  2
              </Grid2>
              <Divider sx={{ height: '100%' }} orientation={"vertical"} flexItem />
              <Grid2 size="grow" sx={{ height: '100%' }}>
                  3
              </Grid2>
          </Grid2>
      )
};

export default Ticket;