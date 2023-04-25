# CorrodingClimate

CorrodingClimate is a command-line tool for retrieving weather information based on the user's current location. The tool uses the Open-Meteo API to fetch weather data.

##  Overview
CorrodingClimate takes a user's location as an input and retrieves its latitude and longitude by making a request to the Geocoding API. It then uses the obtained coordinates to request the current weather data from the OpenMetro API. Finally, it prints the temperature of the given location to the console.

## Getting Started
To get started with CorrodingClimate, follow these steps:

- Clone the repository to your local machine.
- Install Rust by following the instructions on the official Rust website.
- Navigate to the directory where you cloned the repository.
- Run ```cargo run``` to start the app.

## Roadmap

- :construction: Implement caching to reduce the number of API requests
- Improve input validation for user location
- Implement caching to reduce the number of API requests
- Add support for additional weather data beyond temperature
- Add more test cases to improve code coverage
- Refactor code to improve maintainability and readability