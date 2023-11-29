// Import necessary modules and structs from the `crate::http` module and standard library.
use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

// Define a `Handler` trait with methods for handling requests and bad requests.
pub trait Handler {
    // Method to handle a valid HTTP request.
    // It takes a reference to a `Request` and returns a `Response`.
    fn handle_request(&mut self, request: &Request) -> Response;

    // Default implementation for handling bad requests.
    // It takes a reference to a `ParseError` and logs the error, then returns a 400 Bad Request response.
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

// Define a `Server` struct to represent an HTTP server.
pub struct Server {
    addr: String, // The address on which the server will listen.
}

// Implement methods for the `Server` struct.
impl Server {
    // Constructor for `Server`. It takes a `String` representing the address and returns a new `Server`.
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    // Method to run the server.
    // It takes a mutable `Handler` trait object which handles incoming requests.
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        // Bind the TCP listener to the specified address. Unwrap is used, which will panic if binding fails.
        let listener = TcpListener::bind(&self.addr).unwrap();

        // Enter an infinite loop to listen for incoming connections.
        loop {
            match listener.accept() {
                // Handle the case where a new connection is successfully established.
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // Create a buffer to read the request into.

                    match stream.read(&mut buffer) {
                        // If reading from the stream is successful...
                        Ok(_) => {
                            // Log the received request for debugging purposes.
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Try to parse the request. If successful, handle the request; otherwise, handle the bad request.
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            // Attempt to send the response back. Log an error if sending fails.
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        // Log an error if reading from the stream fails.
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                // Log an error if accepting a new connection fails.
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
