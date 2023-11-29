// Standard library imports
use std::io::Read; // For reading from the TCP stream
use std::net::TcpListener; // To listen for TCP connections
use crate::http::Request; // Importing the Request struct from your own `http` module

// Definition of the Server struct
pub struct Server {
    addr: String, // The address where the server will listen
}

// Implementation block for the Server struct
impl Server {
    // Constructor function for Server
    // Takes a String address and returns a new Server instance
    pub fn new(addr: String) -> Server {
        Server { addr }
    }

    // Function to start the server
    // &self indicates that this function borrows the Server instance immutably
    pub fn run(&self) {
        println!("Listening on {}", self.addr);

        // Binding the server to the specified address
        // unwrap_or_else is used to handle potential errors during binding
        let listener = TcpListener::bind(&self.addr).unwrap_or_else(|error| {
            panic!("Failed to bind to address: {}", error);
        });

        // Infinite loop to keep the server running
        loop {
            // Accepting incoming connections and handling them
            match listener.accept() {
                // If a connection is successfully established
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024]; // A buffer to store incoming data, 1024 bytes in size

                    // Reading data from the stream into the buffer
                    match stream.read(&mut buffer) {
                        // If the read operation is successful
                        Ok(_) => {
                            // Logging the incoming request data, converting buffer to a UTF-8 string
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            
                            // Attempt to parse the request using the Request::try_from implementation
                            match Request::try_from(&buffer[..]) {
                                // If parsing is successful
                                Ok(request) => {
                                  //  buffer[1] = 0;
                                  //  let a = request;
                                    // Handle the request here (logic to be implemented)
                                },
                                // If parsing fails
                                Err(e) => println!("Failed to parse request: {}", e),
                            }
                        }
                        // If there's an error in reading from the stream
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                // If there's an error in accepting the connection
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
