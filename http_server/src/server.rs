use std::io::Read;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::TcpListener;
use crate::http::Request;

pub struct Server {
        addr: String,
}
    
// Implementation of Server struct
impl Server{
    // new method that takes in a string and returns a new instance of server Server could be Self too ;)
        pub fn new(addr: String) -> Server {
            // return a new server with the address
                Server {
                    // dont set because the parameter is called addr in struct
                    addr
                }
        }
        
        // run method that takes in a server and returns nothing
        // can take ownership of not - &self
        pub fn run(&self) {
            // print out the address
             println!("Listening on {}", self.addr);

            // bind the address to a listener & pass a ref to the address
             let listener = TcpListener::bind(&self.addr);
            loop {

                // match code will no compile unless all variants are covered
                // ok and error 
                match listener.accept() {
                    //tuple for TCPlistener and SocketAddr
                    //bytes from client - read method on TCP listener
                    Ok((mut stream, _)) => {
                        // read accepts an array [u8]
                        // 1024 bytes
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer) {
                            Ok(_) => {
                                //utf8 lossy accepts buffer with bytes and converts to a string
                                // even invalid characters.
                                println!("Recieved a request: {}", String::from_utf8_lossy(&buffer));

                                match Request::try_from(&buffer[..]) {
                                    Ok(request) => {},
                                    Err(e) => println!("failed to parse request: {}", e)
                                }
                                let res: &Result<Request, > &buffer[..].try_into();

                            }
                            Err(e) => println!("Failed to read from connection: {}", e),
                        }
                    }
                    Err(e) => println!("oh, nooo {}", e);
                }

            }
        } 
    }
