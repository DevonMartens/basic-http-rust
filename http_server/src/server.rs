use std::net::TcpListener;

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
                match listener.accept(){
                    //tuple for TCPlistener and SocketAddr
                    Ok(tup) => {
                        let a = 5;
                        println!("coooool");
                    },
                    Err(e) => println!("oh, nooo {}", e);
                }
                //let res = listener.accept(); 

                if res.is_err() {
                    continue;
                } 

                let (stream, addr) = res.unwrap();
            }
        } 
    }
