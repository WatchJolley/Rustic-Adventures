# Rusty-Listener

This is a project for experimenting with Rust's TCP listener functionality.

## Description

This code sets up a server to listen on port 8080 for incoming TCP connections. When a client connects, the server spawns a new thread to handle the connection. The new thread reads data from the client and sends it to the main thread using Rust's channel communication primitive.

## Goal: Allow clients to send messages to all other connected clients

### Steps:
- [ ] Add a `broadcast` function that sends a message to all connected clients
- [ ] Add a `receiver` that listens for messages from clients and sends them to the `broadcast` function
- [ ] Modify the `handle_client` function to call the `receiver` in a loop
- [ ] Modify the `handle_client` function to write messages to the client