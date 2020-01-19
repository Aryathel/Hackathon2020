import React from "react";
import { makeStyles } from "@material-ui/core/styles";
import Grid from "@material-ui/core/Grid";
import ToggleButton from "@material-ui/lab/ToggleButton";
import ToggleButtonGroup from "@material-ui/lab/ToggleButtonGroup";
import TextField from "@material-ui/core/TextField";
import Autocomplete from "@material-ui/lab/Autocomplete";
import DateFnsUtils from "@date-io/date-fns";
import Fab from "@material-ui/core/Fab";
import SearchIcon from "@material-ui/icons/Search";
import CustomAutoComplete from "../../components/CustomAutoComplete";
import Button from "@material-ui/core/Button";

import {
  MuiPickersUtilsProvider,
  KeyboardTimePicker,
  KeyboardDatePicker,
  DatePicker
} from "@material-ui/pickers";

const useStyles = makeStyles({
  depositContext: {
    flex: 1
  }
});

const FlightsFilter = props => {
  const classes = useStyles();
  const [alignment, setAlignment] = React.useState("left");
  const handleChange = (event, newAlignment) => {
    if (newAlignment === "left") {
      props.setRoundTrip(true);
    } else {
      props.setRoundTrip(false);
    }
    setAlignment(newAlignment);
  };

  const [selectedStartDate, setSelectedStartDate] = React.useState(
    new Date("2020-01-30T21:11:54")
  );

  const [selectedEndDate, setSelectedEndDate] = React.useState(
    new Date("2020-01-30T21:11:54")
  );

  const handleStartDateChange = date => {
    console.log(date);
    setSelectedStartDate(date);
    props.setStartDate(date);
  };

  const handleEndDateChange = date => {
    console.log(date);
    setSelectedEndDate(date);
    props.setEndDate(date);
  };

  return (
    <form onSubmit={props.getFormData}>
      <Grid container spacing={2} direction="column" alignItems="center">
        <Grid item>
          <ToggleButtonGroup
            size="large"
            value={alignment}
            exclusive
            onChange={handleChange}
          >
            <ToggleButton key={1} value="left">
              Round Trip
            </ToggleButton>
            ,
            <ToggleButton key={2} value="right">
              One Way
            </ToggleButton>
            ,
          </ToggleButtonGroup>
        </Grid>
        <Grid item>
          <Grid container justify="center" spacing={6}>
            <Grid key={1} item>
              <CustomAutoComplete
                key={"origin"}
                type={"Origin"}
                wasm={props.wasm}
              />
            </Grid>
            <Grid key={"destination"} item>
              <CustomAutoComplete
                key={"destination"}
                type={"Destination"}
                wasm={props.wasm}
              />
            </Grid>
            <Grid key={3} item>
              <MuiPickersUtilsProvider utils={DateFnsUtils}>
                <DatePicker
                  value={selectedStartDate}
                  onChange={handleStartDateChange}
                />
                <DatePicker
                  value={selectedEndDate}
                  onChange={handleEndDateChange}
                />
              </MuiPickersUtilsProvider>
            </Grid>
            <Grid key={4} item>
              <Button
                variant="contained"
                type="submit"
                color="primary"
                disableElevation
              >
                Submit
              </Button>
            </Grid>
          </Grid>
        </Grid>
      </Grid>
    </form>
  );
};

export default FlightsFilter;

const airports = [
  { title: "Charles De Gaulle Intl", city: "Paris", country: "France (CDG)" },
  { title: "Charles De Gaulle Intl", city: "Paris", country: "France (CDG)" },
  { title: "Charles De Gaulle Intl", city: "Paris", country: "France (CDG)" }
];
