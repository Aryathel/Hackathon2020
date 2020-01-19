import "isomorphic-fetch";
import React from "react";
import TextField from "@material-ui/core/TextField";
import Autocomplete from "@material-ui/lab/Autocomplete";
import CircularProgress from "@material-ui/core/CircularProgress";
import throttle from "lodash/throttle";
import flattenDeep from "lodash/flattenDeep";
import map from "lodash/flattenDeep";
import { instantiateStreaming } from "assemblyscript/lib/loader";

function sleep(delay = 0) {
  return new Promise(resolve => {
    setTimeout(resolve, delay);
  });
}

const CustomAutoComplete = props => {
  const [wasm, setWasm] = React.useState({});
  const [open, setOpen] = React.useState(false);
  const [inputValue, setInputValue] = React.useState("");
  const [options, setOptions] = React.useState([]);
  const loading = open && options.length === 0;

  const handleChange = async event => {
    setInputValue(event.target.value);
    wasm.query_current_search &&
      props.wasm.query_current_search(event.target.value).then(results => {
        if (!results) {
          return;
        }
        const airports = JSON.parse(results);
        if (true) {
          console.log(airports.data);

          setOptions(
            airports.data.map(airport => {
              return { name: airport };
            })
          );
        }
      });
  };

  const fetchData = React.useMemo(
    () => throttle(async (input, callback) => {}, 200),
    []
  );

  React.useEffect(async () => {
    try {
      const test = await import("external");
      console.log(test);
      setWasm(test);
    } catch (err) {
      console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
    }
  }, []);

  React.useEffect(() => {
    let active = true;

    if (!loading) {
      return undefined;
    }
    fetchData({ input: inputValue }, async results => {
      const airports = await results.json();
      if (active) {
        console.log(airports.data);

        setOptions(
          airports.data.map(airport => {
            return { name: airport };
          })
        );
      }
    });

    return () => {
      active = false;
    };
  }, [inputValue, fetchData, loading]);

  React.useEffect(() => {
    if (!open) {
      setOptions([]);
    }
  }, [open]);

  return (
    <Autocomplete
      id={`autcomp-${props.type}`}
      style={{ width: 250 }}
      open={open}
      onOpen={() => {
        setOpen(true);
      }}
      onClose={() => {
        setOpen(false);
      }}
      getOptionSelected={(option, value) => option.name === value.name}
      getOptionLabel={option => option.name}
      autoComplete
      options={options}
      loading={loading}
      disableOpenOnFocus
      renderInput={params => (
        <TextField
          {...params}
          label={`${props.type}`}
          fullWidth
          name={props.type}
          onChange={handleChange}
          variant="outlined"
          InputProps={{
            ...params.InputProps,
            endAdornment: (
              <React.Fragment>
                {loading ? (
                  <CircularProgress color="inherit" size={20} />
                ) : null}
                {params.InputProps.endAdornment}
              </React.Fragment>
            )
          }}
        />
      )}
    />
  );
};

export default CustomAutoComplete;
