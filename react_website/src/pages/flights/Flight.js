import React, { Component } from "react";
import { makeStyles } from "@material-ui/core/styles";
import Grid from "@material-ui/core/Grid";
import Paper from "@material-ui/core/Paper";
import Typography from "@material-ui/core/Typography";
import ButtonBase from "@material-ui/core/ButtonBase";
import Icon from "@material-ui/core/Icon";
import FlightIcon from "@material-ui/icons/Flight";
import Divider from "@material-ui/core/Divider";
import { StepConnector } from "@material-ui/core";

const useStyles = makeStyles(theme => ({
  root: {
    flexGrow: 1
  },
  paper: {
    padding: theme.spacing(2),
    margin: "auto"
  },
  image: {
    width: 128,
    height: 128
  },
  img: {
    margin: "auto",
    display: "block",
    maxWidth: "100%",
    maxHeight: "100%"
  }
}));

const Flight = props => {
  const classes = useStyles();
  return (
    <div className={classes.root}>
      <Paper className={classes.paper}>
        <Grid container spacing={1}>
          <Grid container direction="column" xs={10} item spacing={3}>
            <Grid container item>
              <Grid container direction="column" xs={3} item>
                <Typography variant="body2" color="textSecondary">
                  February 01, 2020
                </Typography>
                <Typography variant="h5">3:35pm</Typography>
                <Typography variant="body2" color="textSecondary">
                  Paris, FR
                </Typography>
              </Grid>
              <Grid xs={6} container spacing={5} justify="center" item>
                <Grid item>
                  <Typography variant="h4" gutterBottom>
                    CDG
                  </Typography>
                </Grid>
                <Grid item>
                  <Grid>
                    <Icon style={{ fontSize: 40 }}>flight</Icon>
                    <Typography variant="body2" color="textSecondary">
                      1h 20m
                    </Typography>
                  </Grid>
                </Grid>
                <Grid item>
                  <Typography variant="h4" gutterBottom>
                    FRA
                  </Typography>
                </Grid>
              </Grid>
              <Grid xs={3} item>
                <Typography variant="body2" color="textSecondary">
                  Flight 1318
                </Typography>
                <Typography variant="h5">4:55pm</Typography>
                <Typography variant="body2" color="textSecondary">
                  Frankfurt, D
                </Typography>
              </Grid>
            </Grid>
            <Grid item xs={12} sm container></Grid>
          </Grid>

          <Grid xs={2} justify="center" time>
            <Typography variant="h4">
              ${`${props.flight.node.fareInfo.totalPrice}`}
            </Typography>
          </Grid>
        </Grid>
      </Paper>
    </div>
  );
};

export default Flight;
