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
Over the course of the 36 hour period, we worked together as a full stack development team to build a website groundwork, both front and back end, as well as a Google Home Mini application and a Python webhook server.

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
   1. During the period where our backend was not able to make requests, Houghton modified the Python Flask server to supply the data to Lucas for testing the front end, albeit in a much slower manner.
1. While Auro dealt with his never-ending battle with WASM compiled Rust and Lucas was busy learning React from scratch, Houghton and Cameron were working on the Google Home again. Cameron was able to create another set of parameter test values to train Dialogflow to recognize another question, while Houghton created the Python webhook endpoints where Dialogflow could connect to the Fly.me API to get the conversation responses.
   1. After this point, the Google Home app was able to hold two fully interactive conversations types. For one, you could ask about a flight from one airport to another at any date in the near future, and get information on the cheapest flights, or get the number of available flights for that trip on that date.
1. Here, with about 10-12 hours left in the competition we felt satisfied with the progress on the Google Home app and turned our focus to the website development. Lucas had already spent the majority of his time building the front end mockups in ReactJS, and it was time to start adding functionality.
   1. Auro was still struggling with the Rust to WASM allowing us to actually send requests. It was an issue. So for the time being, we were still using the temporary Python endpoints.
   1. The first thing that we wanted to do was be able to autofill the airport search bars on the website with relevant airports as the user modified and narrowed or broadened their search. To do this, Houghton built another GraphQL request which would search for all relevant airports by the string the user had been typing in, then returned the list of relevant airports.

# Try It Yourself
There are several parts to being able to try out our program itself, so here is some setup instructions. As of right now the setup only includes the Google Home application

**Google Home App**
Download the `.zip` file from [here](https://github.com/HeroicosHM/Hackathon2020/blob/master/GoogleAssistant-rough-testing.zip) and store it somewhere where you will remember where it is. From there go to the [Dialogflow website](https://dialogflow.com/), sign into the console, and then make sure you are in an app. In the top left of the window, you should see a settings gear icon, click on that. At the top of the screen, under the app name, there is an `Import an Export` tab. Select `Import From Zip` from the options, then find the zip file and bring it into the project. You have now set up our Dialogflow application. In order to make the app work the ret of the way, you will need to set up the Python Flask server.

**Python Flask Server**
After cloning the repository, open the `DialogWebRequest` folder and edit the `flaskSampleHook.py` file. You will need to have a Bearer token for the Fly.me API in order for this to work. You must replace the existing token in the `headers` section at the top of the file with your own bearer token. For our purposes, we used [Heroku](https://www.heroku.com/) to host our Flask application. Once you have a Heroku app created, the [Git](https://git-scm.com/) command line took installed, and the [Heroku CLI](https://devcenter.heroku.com/articles/heroku-cli#download-and-install) installed, open a command line in the `DialogWebRequest` folder, and enter in the commands
```
git init
heroku git:remote -a your-heroku-app-name
git add .
git commit -am "initial commit"
git push heroku master
```
In order to enable the Google Home Dialogflow to work with your new Heroku Python server, return to your app in Dialogflow, open the `Fulfillment` tab on the left of the screen, and enter this link into the `URL` section:
```
https://your-heroku-app-name.herokuapp.com/
```
Once you have done this and saved the setting, you can click on the `Google Assistant` link on the right hand side of the screen to try out the fully functioning app.

# Challenges
* We hit a lot of challenges in our project, especially considering how little experience each of us had with what we were working on. Here are a few of the ones that really hit us hard over the course of the 36 hours.
   * The biggest one: Getting our web application to actually make a web request in WASM directly in the browser. With Web Assembly only being available in most browsers for under a year, there is still a large amount of developing support, and we hit several major snags on things that Rust did not yet have the capability to do.
   * Another major one was getting the website to actually use Web Assembly, at first we attempted to use a standard JavaScript library. It didn't go well. Then we tried an angular site, and this was also just not working. We were getting errors left and right. Eventually we found a solution using ReactJS. However, that meant that Lucas would now need to write the entire website in ReactJS, which he had never touched previously.
   * We had a really hard time starting the project. We had some APIs, some information, and absolutely no clue what to do with it. The projects this year were significantly more vague than the previous, which made it hard for us to get started. Eventually we solved this by just getting started. We just messed around with what we had, and let it lead us to a project.

## A Note From the Members
While the rundown of our project may have seen pretty quick and efficient, don't be fooled. We struggled the whole time. A lot. Houghton had never used GraphQL, the basis of our whole project; our front end developer, Lucas, had never used ReactJS before, but built and entire site out of it in one weekend; Cameron had never worked with a Google Home before; Auro had never dealt with Web Assembly compiled from Rust. These events are learning experiences, they introduce you to new and exciting things. Don't be afraid to put yourself out there and challenge yourself. It isn't about winning, it's about the experience.
