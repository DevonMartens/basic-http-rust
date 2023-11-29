// Import necessary modules and structs from the super module and standard library.
use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;

// Define a `WebsiteHandler` struct to handle website requests.
pub struct WebsiteHandler {
    public_path: String, // The base directory from which to serve files.
}

// Implement methods for `WebsiteHandler`.
impl WebsiteHandler {
    // Constructor for `WebsiteHandler`. Takes a `String` representing the base directory.
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    // Private method to read a file from the file system.
    // Takes a relative file path and returns an `Option<String>` with the file contents.
    fn read_file(&self, file_path: &str) -> Option<String> {
        // Construct the full path by appending the provided `file_path` to `public_path`.
        let path = format!("{}/{}", self.public_path, file_path);

        // Attempt to resolve the full path to its canonical form to prevent directory traversal attacks.
        match fs::canonicalize(path) {
            // If the path is successfully canonicalized...
            Ok(path) => {
                // Check if the canonicalized path starts with `public_path` to ensure it's not outside the base directory.
                if path.starts_with(&self.public_path) {
                    // Read the file to a String and return it if successful.
                    fs::read_to_string(path).ok()
                } else {
                    // Log a warning if a directory traversal attack is attempted.
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            // Return None if there's an error in path resolution.
            Err(_) => None,
        }
    }
}

// Implement the `Handler` trait for `WebsiteHandler`.
impl Handler for WebsiteHandler {
    // Method to handle incoming HTTP requests.
    fn handle_request(&mut self, request: &Request) -> Response {
        // Match on the HTTP method of the request.
        match request.method() {
            // Handle GET requests.
            Method::GET => match request.path() {
                // Serve `index.html` for the root path.
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                // Serve `hello.html` for the `/hello` path.
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                // For other paths, attempt to serve the corresponding file.
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            // Respond with 404 Not Found for non-GET methods.
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
