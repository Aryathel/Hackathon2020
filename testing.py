# Author: Houghton Mayfield                                                             (moral support: Cam ｡◕‿◕｡)
import requests
import json
import datetime

headers = {
    "authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJyb2xlcyI6WyJjb21wYW55LWFkbWluIiwidHJhdmVsX3VzZXIiXSwiaXNzIjoibWUuZmx5IiwiZGVmYXVsdFRyYXZlbGVyUGVyc29uSWQiOiIxMTIiLCJ1c2VySWQiOiI1NTViOWFhMS0wNjMwLTQ1ZDEtYWIxOS01ZTZhNjViMmZkYTUiLCJwdWJsaWNfZXhwaXJlc19lbSI6IjE1Nzk1MDg1NzcwNzciLCJ1c2VyQ29tcGFueUpzb24iOiJ7XCJjb21wYW55SWRcIjozOSxcIm1lbWJlcklkXCI6MTY2LFwiZW5hYmxlZE1lbWJlclwiOnRydWUsXCJhZG1pbmlzdHJhdG9yXCI6dHJ1ZSxcImVuYWJsZWRDb21wYW55XCI6dHJ1ZSxcImNvbnRyYWN0U2lnbmVkXCI6dHJ1ZX0iLCJhdXRoMElkIjoiZ29vZ2xlLW9hdXRoMnwxMTA0MzgzNzE5MzY1ODAyOTU4OTYiLCJwcm9maWxlSWQiOiI5MyIsInNjb3BlIjoiZmx5bWVhcGlkZXYiLCJkYklkIjoiOTMiLCJleHAiOjE1ODIwMTQxNzcsImlhdCI6MTU3OTQyMjE3NiwiZW1haWwiOiJob3VnaHRvbmF3ZUBnbWFpbC5jb20ifQ.txee4ebEPIohbk-Vx35_ZaKVuIvW68AZSilkYQOgW6Q",
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
variables = {
    "fromCode": "FRA",
    "toCode": "FCO",
    "startDate": (datetime.datetime.now() + datetime.timedelta(days = 1)).strftime("%Y-%m-%d"),
    "endDate": (datetime.datetime.now() + datetime.timedelta(days = 9)).strftime("%Y-%m-%d"),
    "roundTrip": True
}

#{'query': airport_query, 'variables': {"query": search}}

flight_query = """
query name($roundTrip: Boolean!, $fromCode: String!, $toCode: String!, $startDate: LocalDate!, $endDate: LocalDate!){
  simpleAirSearch(input: {
    stops: [
      { portCode: $fromCode, earliestDate: $startDate }
      { portCode: $toCode, earliestDate: $endDate }
    ]
    returnsToOrigin: $roundTrip
  }) {
    select {
      products {
        edges {
          node {
            fareInfo { totalPrice, currency }
            ods {
                originDisplayTime
                formattedOriginTime(pattern: "MMMM dd, yyyy")
            }
          }
        }
      }
    }
  }
}
"""

fromCode = "FRA"
toCode = "FCO"
roundTrip = False
tripLength = 3 # in days

session = requests.session()

flightDataSet = {}
numTrips = 0
for i in range(0, 30):
    originDate = (datetime.datetime.now() + datetime.timedelta(days = i)).strftime("%Y-%m-%d")
    print(originDate)
    endDate = (datetime.datetime.now() + datetime.timedelta(days = tripLength + i)).strftime("%Y-%m-%d")
    variables = {
        "fromCode": fromCode,
        "toCode": toCode,
        "startDate": originDate,
        "endDate": endDate,
        "roundTrip": roundTrip
    }
    s = requests.post("https://dev.fly.me/api/graphql", json = {'query': flight_query, 'variables': variables}, headers = headers)
    print(s.status_code)
    if s.status_code == 200:
        #try:
        data = json.loads(s.text)
        data = data['data']['simpleAirSearch']['select']['products']['edges']
        flightDataSet[originDate] = []
        for sec in data:
            flightDataSet[originDate].append({
                "totalPrice": sec['node']['fareInfo']['totalPrice'],
                "currency": sec['node']['fareInfo']['currency'],
                "originDisplayTime": sec['node']['ods'][0]['originDisplayTime'],
                "formattedOriginTime": sec['node']['ods'][0]['formattedOriginTime']
            })
            numTrips += 1
        #except:
        #    print('error')
    else:
        print('failed')

print(json.dumps(flightDataSet, indent = 4))
print(f"Loaded {numTrips} flights.")

"""
print(s.status_code)
data = json.loads(s.text)
print(json.dumps(data, indent = 4))
flights = data['data']['simpleAirSearch']['select']['products']['edges']
print("Number of flights:", len(flights))
"""
