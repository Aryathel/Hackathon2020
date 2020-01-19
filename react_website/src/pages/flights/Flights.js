import React, { Component } from "react";
import Grid from "@material-ui/core/Grid";
import Paper from "@material-ui/core/Paper";
import Chart from "./Chart";
import Deposits from "./Deposits";
import Orders from "./Orders";
import FlightsFilter from "./FlightsFilter";
import Flight from "./Flight";

class Flights extends Component {
  constructor(props) {
    super(props);
    this.state = {
      wasm: {},
      startDate: undefined,
      endDate: undefined,
      roundTrip: true,
      flights: []
    };
  }

  componentDidMount() {
    this.loadWasm();
  }

  setRoundTrip = value => {
    this.setState({ ...this.state, roundTrip: value });
  };

  setStartDate = date => {
    this.setState({ ...this.state, startDate: date });
  };

  setEndDate = date => {
    this.setState({ ...this.state, endDate: date });
  };

  submitForm = async e => {
    e.preventDefault();
    const startDate = this.state.startDate.toISOString().substring(0, 10);
    const endDate = this.state.endDate.toISOString().substring(0, 10);
    const origin = e.target.elements.Origin.value.slice(-4).substring(0, 3);
    const destination = e.target.elements.Destination.value
      .slice(-4)
      .substring(0, 3);

    const api_call = await fetch(
      `https://hack-2020-flask.herokuapp.com/flight_search?airports=${origin},${destination}&dates=${startDate},${endDate}&round-trip=${this.state.roundTrip}`
    );
    const data = await api_call.json();
    this.setState({
      ...this.state,
      flights: data.data.simpleAirSearch.select.products.edges
    });
  };

  loadWasm = async () => {
    try {
      const wasm = await import("external");
      this.setState({ wasm });
    } catch (err) {
      console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
    }
  };

  render() {
    const { wasm = {} } = this.state;
    return (
      <Grid container spacing={3}>
        <Grid item xs={12} md={12} lg={12}>
          <Paper className={this.props.fixedHeightPaper}>
            <FlightsFilter
              wasm={wasm}
              getFormData={this.submitForm}
              setEndDate={this.setEndDate}
              setStartDate={this.setStartDate}
              setRoundTrip={this.setRoundTrip}
            />
          </Paper>
        </Grid>

        <Grid item container direction="column" spacing={2} xs={12}>
          {this.state.flights &&
            this.state.flights.map(flight => <Flight flight={flight} />)}
        </Grid>
        {/* Recent Orders */}
        <Grid item xs={12}>
          <Paper className={this.props.classes.paper}>
            <Orders />
          </Paper>
        </Grid>
      </Grid>
    );
  }
}

export default Flights;
