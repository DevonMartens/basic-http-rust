fn main() {
    // new instance of server - pass ip and port
    //Server in this case is a struct
    // new created in impl block
    let server = Server::new("127.0.0.1:8080".to_string());
    // call run method on server
    server.run();
}

// Struct that holds ip and port as a string
struct Server {
    addr: String,
}

// Implementation of Server struct
impl Server{
    // new method that takes in a string and returns a new instance of server Server could be Self too ;)
    fn new(addr: String) -> Server {
        // return a new server with the address
        Server {
            // dont set because the parameter is called addr in struct
            addr
        }
    }

    // run method that takes in a server and returns nothing
    // can take ownership of not - &self
    fn run(&self) {
        // print out the address
        println!("Listening on {}", self.addr);
    } 
}