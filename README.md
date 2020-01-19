# Hackathon 2020
This is the repository for Houghton Mayfield, Lucas Gompou, Cam Larson, and Aurorans Solis as we take on Hack AZ 2020.

## The Challenge
We decided to take on the [Fly.me](https://fly.me/) challenge, as well as participating in the [American Airlines challenge](https://github.com/AmericanAirlines/Flight-Engine/wiki/Hack-Arizona-2020) as a sub-category.

* For the Fly.me challenge, we were given access to their GraphQL API.
  * The GraphQL API provides a large amount of information about flight planning, prices, booking, and other such resources, including Hotel booking.
  * Given access to this resource, we were given free reign to make something interesting with it.

* For American Airlines, we were challenged to use an API relating to airlines in an interesting and innovative way.
  * Because we already were using the Fly.me API, we decided that we could also submit the project for this challenge as well.

# Our Journey
Over the course of the 36 hour period, we worked together as a full stack development team to build a website groundwork, both front and back end, as well as a Googe Home Mini application and a Python webhook server.

1. The start of our adventure was with establishing a connection too the GraphQL API provided by Fly.me and seeing what we were provided with.
   1. None of our members had used GraphQL previously, so it was quite the adventure learning out how to configure requests properly.
   1. Once we learned how to use GraphQL to access the API, it was time to start brainstorming based on the information we were able to get.
1. After discussion, we had two ideas that stuck with us:
   1. We would make a website which displayed flight information that could be directly searched and populated from the API. We wanted backend for this website to be written in Web Assembly.
   1. At the same time, we also would create a Google Home application that would allow us to ask questions about various flights, such as "What is the cheapest flight from Sky Harbor Airport to Orlando International Airport on the 22nd?"
1. To being with, Houghton worked in Python to develop the various GraphQL API requests that would need to be made. Cameron started work on setting up the Google Home (easier said than done), and Lucas began work on the framework for the website.
   1. Once we had a set of requests being developed so we largely knew what information we were going to be able to get, Aurorans began to work on creating WASM (Web Assembly) binaries from Rust. These binaries compile machine code on the browser side, allowing for more efficiency than previously possible with standard web browsing.
   1. While the Rust backend and front end frameworks were being developed, Cameron was connecting the Google Home to Dialogflow, a program which uses machine learning to pull parameters that you train it to find from a sentence. Once the parameters were pulled from the Google Home, they could be sent via webhook for a fulfillment response.
   1. While Cameron set up the Dialogflow, Houghton set up a Python Flask application on Heroku, which would receive requests from Dialogflow, use the given parameters to query the Fly.me API to get the requested information, then format a response and return it to Dialogflow. Once the response was received, the Google Home would speak the results to the user. At first we started with half of a simple question, then gradually built up one fully interactive question.
1. At this point, we were struggling with getting to make web requests from the WASM backend of the website. Rust would natively run the request and receive a response perfectly, but after compiling to machine code, we found that the authorization header which allowed us to make requests would simply ~~vanish~~.
   1. Too make a long time troubleshooting short, we eventually discovered, many hours later, that the CORS policies in browsers were limiting the request's capabilities when being launched from within the browser. To solve this problem, we created an internal proxy system within the Node server which was serving the bundled ReactJS front end of the website. This proxy system would relay outgoing requests through an internal proxy, successfully dodging the CORS protocol.
