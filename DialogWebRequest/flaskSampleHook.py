from flask import Flask
from flask import request
from flask import Response
from flask_cors import CORS
import json
import gunicorn
import requests
import datetime

app = Flask(__name__)
cors = CORS(app)

session = requests.Session()
session.headers = {
    "authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJyb2xlcyI6WyJjb21wYW55LWFkbWluIiwidHJhdmVsX3VzZXIiXSwiaXNzIjoibWUuZmx5IiwiZGVmYXVsdFRyYXZlbGVyUGVyc29uSWQiOiIxMTIiLCJ1c2VySWQiOiI1NTViOWFhMS0wNjMwLTQ1ZDEtYWIxOS01ZTZhNjViMmZkYTUiLCJwdWJsaWNfZXhwaXJlc19lbSI6IjE1Nzk1MDg1NzcwNzciLCJ1c2VyQ29tcGFueUpzb24iOiJ7XCJjb21wYW55SWRcIjozOSxcIm1lbWJlcklkXCI6MTY2LFwiZW5hYmxlZE1lbWJlclwiOnRydWUsXCJhZG1pbmlzdHJhdG9yXCI6dHJ1ZSxcImVuYWJsZWRDb21wYW55XCI6dHJ1ZSxcImNvbnRyYWN0U2lnbmVkXCI6dHJ1ZX0iLCJhdXRoMElkIjoiZ29vZ2xlLW9hdXRoMnwxMTA0MzgzNzE5MzY1ODAyOTU4OTYiLCJwcm9maWxlSWQiOiI5MyIsInNjb3BlIjoiZmx5bWVhcGlkZXYiLCJkYklkIjoiOTMiLCJleHAiOjE1ODIwMTQxNzcsImlhdCI6MTU3OTQyMjE3NiwiZW1haWwiOiJob3VnaHRvbmF3ZUBnbWFpbC5jb20ifQ.txee4ebEPIohbk-Vx35_ZaKVuIvW68AZSilkYQOgW6Q",
    "content-type": "application/json"
}

@app.route('/', methods = ['GET', 'POST', 'DELETE'])
def hello():
    req = request.get_json(force=True)
    type_of_search = req.get('queryResult').get('parameters').get('FlightData')
    airport_param = req.get('queryResult').get('parameters').get('airport')
    date = str(req.get('queryResult').get('parameters').get('date-time')).split('T')[0]
    givenDate = True
    if not date:
        date = datetime.datetime.now().strftime("%Y-%m-%d")
        givenDate = False

    print(type_of_search)
    print(airport_param)
    print(date)

    if len(airport_param) == 2 and "cheapest_flight" in type_of_search:
        query = """
        query name($fromCode: String!, $toCode: String!, $startDate: LocalDate!) {
          simpleAirSearch(input: {
            stops: [
              { portCode: $fromCode, earliestDate: $startDate}
              { portCode: $toCode }
            ]
            returnsToOrigin: false
          }) {
            select {
              products(first: 20) {
                edges {
                  node {
                    fareInfo { totalPrice, currency, f_total_price: formattedPrice(type: TOTAL) }
                    ods {
                        formattedOriginTime(pattern: "MMMM dd, yyyy")
                    }
                  }
                }
              }
            }
          }
        }
        """
        variables = {
            "fromCode": airport_param[0]['IATA'],
            "toCode": airport_param[1]['IATA'],
            "startDate": date
        }

        s = session.post("https://dev.fly.me/api/graphql", json = {'query': query, 'variables': variables})
        if s.status_code == 200:
            flights = json.loads(s.text)['data']['simpleAirSearch']['select']['products']['edges']
            print(flights)
            if len(flights) > 0:
                cheapest = flights[0]['node']['fareInfo']['totalPrice']
                currency = flights[0]['node']['fareInfo']['currency']
                cheapest_text = flights[0]['node']['fareInfo']['f_total_price']
                date = flights[0]['node']['ods'][0]['formattedOriginTime']
                for i in flights:
                    if i['node']['fareInfo']['totalPrice'] < cheapest:
                        cheapest = i['node']['fareInfo']['totalPrice']
                        cheapest_text = i['node']['fareInfo']['f_total_price']
                        date = i['node']['ods'][0]['formattedOriginTime']

                if givenDate:
                    speech_msg = f"The cheapest flight on {date} from {airport_param[0]['name']} to {airport_param[1]['name']} costs {cheapest} {currency}."
                    text_msg = f"The cheapest flight on {date} from {airport_param[0]['name']} to {airport_param[1]['name']} costs {cheapest_text}."
                else:
                    speech_msg = f"The cheapest flight from {airport_param[0]['name']} to {airport_param[1]['name']} costs {cheapest} {currency} and is on {date}."
                    text_msg = f"The cheapest flight from {airport_param[0]['name']} to {airport_param[1]['name']} costs {cheapest_text} and is on {date}."
            else:
                speech_msg = "There are not any flights scheduled for those locations. Please try again."
                text_msg = "There are not any flights scheduled for those locations. Please try again."
        else:
            speech_msg = "I am sorry, an error occured. Please try again."
            text_msg = "I am sorry, an error occured. Please try again."
    elif len(airport_param) == 2 and "how_many" in type_of_search:
        flight_query = """
        query name($roundTrip: Boolean!, $fromCode: String!, $toCode: String!, $startDate: LocalDate!){
          simpleAirSearch(input: {
            stops: [
              { portCode: $fromCode, earliestDate: $startDate }
              { portCode: $toCode }
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

        variables = {
            "fromCode": fromCode,
            "toCode": toCode,
            "startDate": date,
            "roundTrip": roundTrip
        }
        s = session.post("https://dev.fly.me/api/graphql", json = {'query': flight_query, 'variables': variables})
        print(s.status_code)
        print(s.text)
        if s.status_code == 200:
            try:
                data = json.loads(s.text)
                data = data['data']['simpleAirSearch']['select']['products']['edges']
                print(data)
                numTrips = 0
                date = data[0]['node']['ods'][0]['formattedOriginTime']
                for sec in data:
                    numTrips += 1
                speech_msg = f"There are {numTrips} flights schedule from {airport_param[0]['name']} to {airport_param[1]['name']} on {date}."
                text_msg = f"There are {numTrips} flights schedule from {airport_param[0]['name']} to {airport_param[1]['name']} on {date}."
            except:
                speech_msg = "I am sorry, an error occured. Please try again."
                text_msg = "I am sorry, an error occured. Please try again."
        else:
            speech_msg = "I am sorry, an error occured. Please try again."
            text_msg = "I am sorry, an error occured. Please try again."
    else:
        speech_msg = "I am sorry, I cannot do that. Please try again."
        text_msg = "I am sorry, I cannot do that. Please try again."
    return(json.dumps({"payload": {"google": {"expectUserResponse": False,"richResponse": {"items": [{"simpleResponse": {"textToSpeech": speech_msg, "displayText":text_msg}}]}}}}))

@app.route('/while_typing', methods = ['GET', 'POST', 'DELETE'])
def active_type():
    text = request.args.get('text')

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
        "query": text
    }

    s = session.post("https://dev.fly.me/api/graphql", json = {'query': airport_query, 'variables': variables})
    if s.status_code == 200:
        try:
            data = json.loads(s.text)
            data = data['data']['airports']['edges']
            airports = {
                "data": []
            }
            for sec in data:
                if 'subSuggestions' in sec['node'].keys():
                    sub_list = [i['selectedText'] for i in sec['node']['subSuggestions']]
                    for item in sub_list:
                        airports['data'].append(item)
                else:
                    airports['data'].append(sec['node']['selectedText'])

            print(airports)
            return airports
        except:
            return 'fail'
    else:
        return str(s.status_code)

@app.route('/flight_search', methods = ['GET', 'POST', 'DELETE'])
def flight_search():
    airports = request.args.get('airports')
    fromCode = airports.split(',')[0]
    toCode = airports.split(',')[1]

    dates = request.args.get('dates')
    fromDate = dates.split(',')[0]
    toDate = dates.split(',')[1]

    if request.args.get('round-trip').lower() == "true":
        roundTrip = True
    else:
        roundTrip = False

    query = """
    query name($roundTrip: Boolean!, $fromCode: String!, $toCode: String!, $fromDate: LocalDate!, $toDate: LocalDate!) {
      simpleAirSearch(input: {
        stops: [
          { portCode: $fromCode, earliestDate: $fromDate }
          { portCode: $toCode, earliestDate: $toDate }
        ]
        returnsToOrigin: $roundTrip
      }) {
        select {
          products(first: 10) {
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

    variables = {
        "fromCode": fromCode,
        "toCode": toCode,
        "fromDate": fromDate,
        "toDate": toDate,
        "roundTrip": roundTrip
    }
    print(variables)

    s = session.post("https://dev.fly.me/api/graphql", json = {'query': query, 'variables': variables})
    print(s.status_code)
    print(s.text)
    if s.status_code == 200:
        #try:
            #data = json.loads(s.text)
            #data = data['data']['airports']['edges']
        return s.text
        #except:
        #    return 'fail'
    else:
        return str(s.status_code)

@app.route('/visual_data', methods=['GET', 'POST', 'DELETE'])
def get_visual_data():
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

    def visual_data():
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
            s = session.post("https://dev.fly.me/api/graphql", json = {'query': flight_query, 'variables': variables})
            print(s.status_code)
            if s.status_code == 200:
                try:
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
                except:
                    print('error')
            else:
                print('failed')

            yield json.dumps(flightDataSet[originDate], indent = 4)
    return Response(visual_data(), mimetype='application/json')
    print(f"Loaded {numTrips} flights.")
