# Steelstack

Steelstack is a full-stack web development project built entirely in Rust. The project consists of a backend built with the Rocket web framework and a frontend built with the Yew framework, connected by a RESTful API.

## Backend

The backend of Steelstack uses Rocket, a fast and flexible web framework that makes it easy to build web applications in Rust. It allows for easy handling of requests and responses, and makes it easy to interact with databases using the Diesel ORM.

## Database

Steelstack uses MySQL as the database solution, which will be run in a container. This provides a convenient and isolated way to manage the database, without needing to install MySQL directly on the host machine. Steelstack uses the Diesel ORM to interact with the MySQL database from the backend.

## Frontend

The frontend of Steelstack uses Yew, a Rust framework for building client-side web applications. Yew is based on the Elm architecture and uses a virtual DOM to manage UI state, making it easy to build reactive, fast, and scalable web applications.

## Working On

Walking through this code to get diesel connected and working with mySQL
https://diesel.rs/guides/getting-started