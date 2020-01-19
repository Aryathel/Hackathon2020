0000...............0use futures::executor::block_on;
use serde::Serialize;
use wasm_bindgen::prelude::*;

const BEARER_AUTH: &'static str =
    "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJyb2xlcyI6W10sImlzcyI6Im1\
lLmZseSIsImRlZmF1bHRUcmF2ZWxlclBlcnNvbklkIjoiMTEyIiwidXNlcklkIjoiNTU1YjlhYTEtMDYzMC00NWQxLWFiMTktNW\
U2YTY1YjJmZGE1IiwicHVibGljX2V4cGlyZXNfZW0iOiIxNTc5NDA3NDk2NjczIiwidXNlckNvbXBhbnlKc29uIjoie1wiY29tc\
GFueUlkXCI6MzksXCJtZW1iZXJJZFwiOjE2NixcImVuYWJsZWRNZW1iZXJcIjp0cnVlLFwiYWRtaW5pc3RyYXRvclwiOmZhbHNl\
LFwiZW5hYmxlZENvbXBhbnlcIjp0cnVlLFwiY29udHJhY3RTaWduZWRcIjp0cnVlfSIsImF1dGgwSWQiOiJnb29nbGUtb2F1dGg\
yfDExMDQzODM3MTkzNjU4MDI5NTg5NiIsInByb2ZpbGVJZCI6IjkzIiwic2NvcGUiOiJmbHltZWFwaWRldiIsImRiSWQiOiI5My\
IsImV4cCI6MTU4MTkxMzA5NiwiaWF0IjoxNTc5MzIxMDk2LCJlbWFpbCI6ImhvdWdodG9uYXdlQGdtYWlsLmNvbSJ9.6AQD77zz\
DSf9E3FD_f6gjcMRskCo_0XueGkNw9QRwXM";

const AIRPORT_QUERY: &'static str = "\
query findAirports($query: String!) {\n\
  airports(query: $query) {\n\
    edges {\n\
      node {\n\
        ...Airport\n\
      }\n\
    }\n\
  }\n\
}\n\
\n\
fragment Airport on AirportSuggestion {\n\
  iataCode\n\
  title\n\
  selectedText\n\
  subSuggestions {\n\
    iataCode\n\
    title\n\
    selectedText\n\
  }\n\
}\
";

#[derive(Serialize)]
struct ApQueryVariables<'a> {
    query: &'a str,
}

#[derive(Serialize)]
struct ApQuery<'a> {
    query: &'static str,
    variables: ApQueryVariables<'a>,
}

impl<'a> ApQuery<'a> {
    fn new(search: &'a str) -> Self {
        ApQuery {
            query: AIRPORT_QUERY,
            variables: ApQueryVariables { query: search },
        }
    }
}

#[wasm_bindgen]
pub fn query_current_search(search: &str) -> String {
    block_on(
        match block_on(
            reqwest::Client::new()
                .post("https://dev.fly.me/api/graphql")
                .header("authorization", BEARER_AUTH)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ApQuery::new(search)).unwrap())
                .send(),
        ) {
            Ok(response) => response,
            Err(_) => return "Error: timed out.".to_string(),
        }
        .text()
    )
    .unwrap_or_else(|_| "Error: failed to get response text.".to_string())
}

/*struct FlightSearch<'a> {
    ap_from: &'a str,
    ap_to: &'a str,
    depart_date: &'a str,
    return_date: Option<&'a str>,
    round_trip: bool,
}*/
