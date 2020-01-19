use crate::BEARER_AUTH;
use reqwest::Client;
use serde::Serialize;
use wasm_bindgen::prelude::*;

const FLIGHT_QUERY: &'static str = "
query name($roundTrip: Boolean!, $fromCode: String!, $toCode: String!, $fromDate: LocalDate!, $toDate: LocalDate!) {
  simpleAirSearch(input: {
    stops: [
      { portCode: $fromCode, earliestDate: $fromDate }
      { portCode: $toCode, earliestDate: $toDate }
    ]
    returnsToOrigin: $roundTrip
  }) {
    select {
      products(first: 1) {
        edges {
          node {
            type
            productId
            fareInfo {
                validatingCarrier, totalPrice, currency
                totalTax
                f_fare: formattedPrice(type: BASE)
                f_total_price: formattedPrice(type: TOTAL)
                f_taxes_and_fees_total: formattedPrice(type: TAXES_AND_FEES)
                fareFamily {
                  seatSelectionIncluded
                  fareFamilyDescription
                }
            }
            ods {
              id
              origin
              originDisplayTime
              destinationDisplayTime
              departureTime: formattedOriginTime(pattern: \"h:mm a\")
              arrivalTime: formattedDestinationTime(pattern: \"h:mm a\")
              formattedOriginTime(pattern: \"MMMM dd, yyyy\")
              formattedDestinationTime(pattern: \"MMMM dd, yyyy\")
              origin
              originDescriptive {
                primaryType {
                    type
                    name
                    code
                }
                  timeZoneCode
                  city: ltv(types: [CITY]) {
                    name
                  }
                  country: ltv(types: [COUNTRY]) {
                    code
                    name
                  }
                  state: ltv(types: [STATE]) {
                    code
                  }
                }
                destination
                destinationDescriptive {
                  primaryType {
                    type
                    name
                    code
                  }
                  timeZoneCode
                  city: ltv(types: [CITY]) {
                    name
                  }
                  country: ltv(types: [COUNTRY]) {
                    code
                    name
                  }
                  state: ltv(types: [STATE]) {
                    code
                  }
              }
              layovers {
                  detail: arrivalPortDescriptive {
                    city: ltv(types: [CITY]) {
                      name
                    }
                    port: ltv(types: [AIRPORT]) {
                      code
                    }
                  }
                  layoverTimeMinutes
              }
              segments {
                flightNumber
                departurePort, departureTime, marketingCarrier
                departurePortDescriptive {
                    primaryType {
                      type
                      name
                      code
                    }
                    timeZoneCode
                }
                departureTerminal
                arrivalPort, arrivalTime
                arrivalPortDescriptive {
                    primaryType {
                      type
                      name
                      code
                    }
                    timeZoneCode
                }
                arrivalTerminal
                aircraft {
                    code
                    shortName
                    longName
                }
                flightTimeMinutes
                layoverTimeMinutes
              }
            }
          }
        }
      }
    }
  }
}
";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[derive(Serialize)]
struct FlightQueryVariables<'a> {
    ap_from: &'a str,
    ap_to: &'a str,
    depart_date: &'a str,
    return_date: Option<&'a str>,
    round_trip: bool,
}

impl<'a> FlightQueryVariables<'a> {
    fn new(
        ap_from: &'a str,
        ap_to: &'a str,
        depart_date: &'a str,
        return_date: Option<&'a str>,
        round_trip: bool,
    ) -> Self {
        FlightQueryVariables {
            ap_from,
            ap_to,
            depart_date,
            return_date,
            round_trip,
        }
    }
}

#[derive(Serialize)]
pub struct FlightQuery<'a> {
    query: &'static str,
    variables: FlightQueryVariables<'a>,
}

impl<'a> FlightQuery<'a> {
    pub fn new(
        ap_from: &'a str,
        ap_to: &'a str,
        depart_date: &'a str,
        return_date: Option<&'a str>,
        round_trip: bool,
    ) -> Self {
        FlightQuery {
            query: FLIGHT_QUERY,
            variables: FlightQueryVariables::new(
                ap_from,
                ap_to,
                depart_date,
                return_date,
                round_trip,
            ),
        }
    }
}

#[wasm_bindgen]
pub async fn flight_query(
    ap_from: String,
    ap_to: String,
    depart_date: String,
    return_date: Option<String>,
    round_trip: bool,
) -> String {
    console_error_panic_hook::set_once();
    let response = match Client::new()
        .post("http://localhost:3000/api/graphql")
        .header("authorization", BEARER_AUTH)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&FlightQuery::new(
                &ap_from,
                &ap_to,
                &depart_date,
                return_date
                    .as_ref()
                    .map_or_else(|| None, |s| Some(s.as_str())),
                round_trip,
            ))
            .unwrap(),
        )
        .send()
        .await
    {
        Ok(response) => match response.text().await {
            Ok(response) => response,
            Err(e) => format!("Error: {}", e),
        },
        Err(e) => format!("Error: {}", e),
    };
    response
}
