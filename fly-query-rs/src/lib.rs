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
query findAirports($query: String!) {
  airports(query: $query) {
    edges {
      node {
        ...Airport
      }
    }
  }
}

fragment Airport on AirportSuggestion {
  iataCode
  title
  selectedText
  subSuggestions {
    iataCode
    title
    selectedText
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
    query: &'a str,
}

#[derive(Serialize)]
pub struct ApQuery<'a> {
    query: &'static str,
    variables: ApQueryVariables<'a>,
}

impl<'a> ApQuery<'a> {
    pub fn new(search: &'a str) -> Self {
        ApQuery {
            query: AIRPORT_QUERY,
            variables: ApQueryVariables { query: search },
        }
    }
}

#[wasm_bindgen]
pub async fn query_current_search(search: String) -> String {
    console_error_panic_hook::set_once();
    let mut request = post("http://localhost:3000/api/graphql")
        .set_header("authorization", BEARER_AUTH)
        .body_json(&ApQuery::new(&search))
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

