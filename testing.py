# Author: Houghton Mayfield                                                             (moral support: Cam ｡◕‿◕｡)
import requests
import json

headers = {
    "authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJyb2xlcyI6W10sImlzcyI6Im1lLmZseSIsImRlZmF1bHRUcmF2ZWxlclBlcnNvbklkIjoiMTEyIiwidXNlcklkIjoiNTU1YjlhYTEtMDYzMC00NWQxLWFiMTktNWU2YTY1YjJmZGE1IiwicHVibGljX2V4cGlyZXNfZW0iOiIxNTc5NDA3NDk2NjczIiwidXNlckNvbXBhbnlKc29uIjoie1wiY29tcGFueUlkXCI6MzksXCJtZW1iZXJJZFwiOjE2NixcImVuYWJsZWRNZW1iZXJcIjp0cnVlLFwiYWRtaW5pc3RyYXRvclwiOmZhbHNlLFwiZW5hYmxlZENvbXBhbnlcIjp0cnVlLFwiY29udHJhY3RTaWduZWRcIjp0cnVlfSIsImF1dGgwSWQiOiJnb29nbGUtb2F1dGgyfDExMDQzODM3MTkzNjU4MDI5NTg5NiIsInByb2ZpbGVJZCI6IjkzIiwic2NvcGUiOiJmbHltZWFwaWRldiIsImRiSWQiOiI5MyIsImV4cCI6MTU4MTkxMzA5NiwiaWF0IjoxNTc5MzIxMDk2LCJlbWFpbCI6ImhvdWdodG9uYXdlQGdtYWlsLmNvbSJ9.6AQD77zzDSf9E3FD_f6gjcMRskCo_0XueGkNw9QRwXM",
    "content-type": "application/json"
}

basic_query = """
{
  profile {
    accountDetails {
      id, accountEmail, displayName
    }
    companyMembership {
      companyId, companyName, memberId, companyAdmin
    }
  }
}
"""


query = """
query name($roundTrip: Boolean!, $fromCode: String!, $toCode: String!) {
  simpleAirSearch(input: {
    stops: [
      { portCode: $fromCode, earliestDate: "2020-01-18"}
      { portCode: $toCode, earliestDate: "2020-01-22"}
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
              departureTime: formattedOriginTime(pattern: "h:mm a")
              arrivalTime: formattedDestinationTime(pattern: "h:mm a")
              formattedOriginTime(pattern: "MMMM dd, yyyy")
              formattedDestinationTime(pattern: "MMMM dd, yyyy")
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
"""

airport_query = """
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
    location {
      ...Locations
    }
  }
  location {
    ...Locations
    coordinates {
      points {
        lat
        lon
      }
    }
  }
}

fragment Locations on Location {
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
"""

variables = {
    "fromCode": "FRA",
    "toCode": "FCO",
    "startDate": "2020-02-02",
    "endDate": "2020-02-05",
    "roundTrip": False
}

variables = {
    "query": "FRA"
}

s = requests.post("https://dev.fly.me/api/graphql", json = {'query': airport_query, 'variables': variables} , headers = headers)

print(s.status_code)
data = json.loads(s.text)
print(json.dumps(data, indent = 4))
