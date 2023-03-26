# Rusty-Listener

This is a project for experimenting with Rust's TCP listener functionality.

## Description

This code sets up a server to listen on port 8080 for incoming TCP connections. When a client connects, the server spawns a new thread to handle the connection. The new thread reads data from the client and sends it to the main thread using Rust's channel communication primitive.