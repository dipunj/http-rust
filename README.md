# HTTP server

A Rust implementation of HTTP 1.1. Head over to main.rs to see an example of how to run the project.

To run the project, run (the server is set to run on 127.0.0.1:4221) and can server multiple HTTP 1.1 requests concurrently.
To customize the application logic, simply implement your own handle_requests() function and pass it to Server(). 

```bash
./your_server
```