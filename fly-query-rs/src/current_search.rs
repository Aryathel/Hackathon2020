use crate::BEARER_AUTH;
use reqwest::Client;
use serde::{de::IgnoredAny, Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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
    data: Vec<String>,
}

impl ApQueryOut {
    fn new(data: Vec<String>) -> Self {
        ApQueryOut { data }
    }

    fn build_from(api_response: String) -> String {
        let deserialized = match serde_json::from_str::<ApQueryResponse>(&api_response) {
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
            data.into_iter()
                .for_each(|item| item.node.flatten(&mut collection));
            let out = ApQueryOut::new(collection);
            match serde_json::to_string(&out) {
                Ok(serialized_out) => serialized_out,
                Err(e) => format!("Error (3): {:?}", e),
            }
        }
    }
}

#[wasm_bindgen]
pub async fn query_current_search(search: String) -> String {
    console_error_panic_hook::set_once();
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
    ApQueryOut::build_from(response)
}
