use reqwest::Client;
use serde::Serialize;
use std::str::from_utf8;
use surf::post;
use wasm_bindgen::prelude::*;

const BEARER_AUTH: &'static str =
    "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJyb2xlcyI6WyJjb21wYW55LWFkbWluIiwidHJhdmVsX3VzZX\
    IiXSwiaXNzIjoibWUuZmx5IiwiZGVmYXVsdFRyYXZlbGVyUGVyc29uSWQiOiIxMTIiLCJ1c2VySWQiOiI1NTViOWFhMS0wN\
    jMwLTQ1ZDEtYWIxOS01ZTZhNjViMmZkYTUiLCJwdWJsaWNfZXhwaXJlc19lbSI6IjE1Nzk1MDg1NzcwNzciLCJ1c2VyQ29t\
    cGFueUpzb24iOiJ7XCJjb21wYW55SWRcIjozOSxcIm1lbWJlcklkXCI6MTY2LFwiZW5hYmxlZE1lbWJlclwiOnRydWUsXCJ\
    hZG1pbmlzdHJhdG9yXCI6dHJ1ZSxcImVuYWJsZWRDb21wYW55XCI6dHJ1ZSxcImNvbnRyYWN0U2lnbmVkXCI6dHJ1ZX0iLC\
    JhdXRoMElkIjoiZ29vZ2xlLW9hdXRoMnwxMTA0MzgzNzE5MzY1ODAyOTU4OTYiLCJwcm9maWxlSWQiOiI5MyIsInNjb3BlI\
    joiZmx5bWVhcGlkZXYiLCJkYklkIjoiOTMiLCJleHAiOjE1ODIwMTQxNzcsImlhdCI6MTU3OTQyMjE3NiwiZW1haWwiOiJo\
    b3VnaHRvbmF3ZUBnbWFpbC5jb20ifQ.txee4ebEPIohbk-Vx35_ZaKVuIvW68AZSilkYQOgW6Q";

const AIRPORT_QUERY: &'static str = "
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
struct ApQueryVariables<'a> {
    fromCode: &'a str,
    toCode: &'a str,
    fromDate: &'a str,
    toDate: &'a str,
    roundTrip: &'a bool
}

#[derive(Serialize)]
pub struct ApQuery<'a> {
    query: &'static str,
    variables: ApQueryVariables<'a>,
}

impl<'a> ApQuery<'a> {
    pub fn new(fromCode: &'a str, toCode: &'a str, fromDate: &'a str, toDate: &'a str, roundTrip: &'a bool) -> Self {
        ApQuery {
            query: AIRPORT_QUERY,
            variables: ApQueryVariables { fromCode: fromCode, toCode: toCode, fromDate: fromDate, toDate: toDate, roundTrip: roundTrip },
        }
    }
}

#[wasm_bindgen]
pub async fn query_current_search(fromCode: String, toCode: String, fromDate: String, toDate: String, roundTrip: Boolean) -> String {
    console_error_panic_hook::set_once();
    let mut request = post("http://localhost:3000/api/graphql")
        .set_header("authorization", BEARER_AUTH)
        .body_json(&ApQuery::new(&fromCode, &toCode, &fromDate, &toDate, &roundTrip))
        .unwrap();
    /*let client = Client::new();
    let request = client.post("/api/graphql")
        .bearer_auth(BEARER_AUTH)
        .json(&ApQuery::new(&search))
        .build()
        .unwrap();*/
    let response = match request.recv_string().await {
        Ok(response) => response,
        Err(e) => return format!("Error: {:?}", e),
    };
    /*let response = match Client::new()
        .post("http://localhost:3000/api/graphql")
        .header("authorization", BEARER_AUTH)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&ApQuery::new(&search)).unwrap())
        .send()
        .await
    {
        Ok(response) => match response.text().await {
            Ok(response) => response,
            Err(e) => format!("Error: {}", e),
        },
        Err(e) => format!("Error: {}", e),
    };*/
    response
}

/*struct FlightSearch<'a> {
    ap_from: &'a str,
    ap_to: &'a str,
    depart_date: &'a str,
    return_date: Option<&'a str>,
    round_trip: bool,
}*/
