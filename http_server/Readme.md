# Rust Server

The focus of this project isn't to build a production ready server but yet to demonstrate a knowledge of rust fundemental. Over the course of the next few months I will be independently building more complex rust apps and explaining them.

### Architecture of the project (Server)

* TCP  (Transmission Control Protocol) Listener - http is transfered over tcp so we need the ability to listen to new tcp connections and read and write bytes.
* Http Parser - Parse data into new useful http data structures. (See websockets and make decsions before listening to blockchain events ;).
* Handler (request handler) - routing logic that executes code/opens files. Then creates a response and sends it back to client. 

This will all run on one thread. We handle one request at a time.

The crate [!net](https://doc.rust-lang.org/std/net/index.html) - provides a TCPListener and a TCPStream 

[!TCP listener](https://doc.rust-lang.org/std/net/struct.TcpListener.html) takes a `bind` which binds to a socket an address listening for TCP connections. These are accepted by calling `accept` or `Incoming` iterator returned by `incoming`.



    ```rust
    use std::net::{TcpListener, TcpStream};

    fn handle_client(stream: TcpStream) {
        // ...
    }

    fn main() -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:80")?;

        // accept connections and process them serially
        for stream in listener.incoming() {
            handle_client(stream?);
        }
        Ok(())
    }
    ```

     - bind returns a special type that wraps the general result.