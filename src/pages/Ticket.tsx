import React, { useState }  from "react";
import {useTicketStore} from "../store/TicketStore.ts";
import {Box, Button, Grid2, Typography} from "@mui/material";


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
          <Box> Ticket </Box>
      )
};

export default Ticket;