use reqwest::Client;
use serde::{de::IgnoredAny, Deserialize, Serialize};
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

#[derive(Deserialize)]
struct ApQueryResponseNode {
    #[serde(rename(deserialize = "iataCode"))]
    _iata_code: IgnoredAny,
    #[serde(rename(deserialize = "title"))]
    _title: IgnoredAny,
    #[serde(rename(deserialize = "selectedText"))]
    selected_text: String,
}

// MG signifies Maybe Group. I don't particularly want to deal with recursive structs, and since
// only a single level of nesting of nodes is possible, I'll just create the additional struct.
#[derive(Deserialize)]
struct ApQueryResponseNodeMG {
    #[serde(rename(deserialize = "iataCode"), skip_deserializing)]
    _iata_code: IgnoredAny,
    #[serde(rename(deserialize = "title"), skip_deserializing)]
    _title: IgnoredAny,
    #[serde(rename(deserialize = "selectedText"))]
    selected_text: String,
    #[serde(rename(serialize = "subSuggestions"), default)]
    subsuggestions: Option<Vec<ApQueryResponseNode>>,
}

impl ApQueryResponseNodeMG {
    fn flatten(self, collection: &mut Vec<String>) {
        if let Some(inner) = self.subsuggestions {
            for item in inner {
                collection.push(item.selected_text);
            }
        } else {
            collection.push(self.selected_text);
        }
    }
}

#[derive(Deserialize)]
struct ApQueryResponseNodeMGWrapper {
    node: ApQueryResponseNodeMG,
}

#[derive(Deserialize)]
struct ApQueryResponseAirports {
    edges: Vec<ApQueryResponseNodeMGWrapper>,
}

#[derive(Deserialize)]
struct ApQueryResponseData {
    airports: ApQueryResponseAirports,
}

#[derive(Deserialize)]
struct ApQueryResponse {
    data: ApQueryResponseData,
}

#[derive(Serialize)]
struct ApQueryOut {
    data: Vec<String>
}

impl ApQueryOut {
    fn new(data: Vec<String>) -> Self {
        ApQueryOut {
            data,
        }
    }
}

#[wasm_bindgen]
pub async fn query_current_search(search: String) -> String {
    console_error_panic_hook::set_once();
    /*let request = post("http://localhost:3000/api/graphql")
        .set_header("authorization", BEARER_AUTH)
        .body_json(&ApQuery::new(&search))
        .unwrap();
    let response = match request.recv_string().await {
        Ok(response) => response,
        Err(e) => return format!("Error (1): {:?}", e),
    };*/
    let response = match Client::new()
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
    };
    let deserialized = match serde_json::from_str::<ApQueryResponse>(&response) {
        Ok(deserialized) => deserialized,
        Err(e) => return format!("Error (2): {:?}", e),
    };
    let data = deserialized.data.airports.edges;
    if data.len() == 0 {
        log("empty response");
        String::new()
    } else {
        let mut collection = Vec::with_capacity(
            data.iter()
                .map(|mg| {
                    mg.node
                        .subsuggestions
                        .as_ref()
                        .map_or_else(|| 1, |inner| inner.len())
                })
                .sum(),
        );
        for item in data {
            item.node.flatten(&mut collection);
        }
        let out = ApQueryOut::new(collection);
        match serde_json::to_string(&out) {
            Ok(serialized_out) => serialized_out,
            Err(e) => format!("Error (2): {:?}", e),
        }
    }
}

/*struct FlightSearch {
    ap_from: &'a str,
    ap_to: &'a str,
    depart_date: &'a str,
    return_date: Option<&'a str>,
    round_trip: bool,
}*/
