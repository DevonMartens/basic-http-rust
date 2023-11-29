#![allow(dead_code)]
use server::Server;
//use http::request::Request;
//use std;

// mod server; // module server in server.rs
mod server;
mod http;

fn main() {

    
    /*  Server instance as a struct
        * - pass ip and port
        * - the new() method is created in impl block
        * - prepended with name Server because Server is a module
    */
    let server = Server::new("127.0.0.1:8080".to_string());

    // Calls run method on server - impl block
    server.run();
}


// // Path: http_server/src/main.rs

