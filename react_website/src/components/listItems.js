import React from "react";
import ListItem from "@material-ui/core/ListItem";
import ListItemIcon from "@material-ui/core/ListItemIcon";
import ListItemText from "@material-ui/core/ListItemText";
import { NavLink } from "react-router-dom";
import FlightIcon from "@material-ui/icons/Flight";
import HotelIcon from "@material-ui/icons/Hotel";

export const mainListItems = (
  <div>
    <NavLink to="/flights">
      <ListItem button>
        <ListItemIcon>
          <FlightIcon />
        </ListItemIcon>
        <ListItemText primary="Flights" />
      </ListItem>
    </NavLink>

    <NavLink to="/hotels">
      <ListItem button>
        <ListItemIcon>
          <HotelIcon />
        </ListItemIcon>
        <ListItemText primary="Hotels" />
      </ListItem>
    </NavLink>
  </div>
);
