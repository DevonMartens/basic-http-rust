// use super::method::Method; // use the method module in the parent module
use super::method::Method; // use the method module in the parent module
use std::convert::TryFrom; // Simple and safe type conversions that may fail in a controlled way under some circumstances.
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html

/* Request struct
- The path in an HTTP request is the portion of the URL that follows the domain name and specifies the location of the resource or endpoint on the server that the client wants to access.
- query string in an HTTP request is a part of the URL that contains data in key-value pairs, typically used to send parameters or additional information to a server for processing.
- Method - enum one of 9 methods see: Request_Type.md
*/

// model request: GET /user?id=10 HTTP/1.1\r\n HEADERS BODY \r\n

pub struct Request {
    path: String,
    query_string: Option<String> , // it is possible to not have we create an option enum 
    method: Method,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
}

impl TryFrom<&[u8]> for Request { 
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

trait Encrypt {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}

impl Encrypt for &[u8] {
    fn encrypt(&self) -> Self{
        unimplemented!()
    }
}
/*
trait Encrypt:

This line starts the definition of a trait named Encrypt. 
In Rust, a trait is similar to an interface in other programming languages. 
It defines functionality that other types (like structs or enums) can implement. 
Essentially, it specifies a set of methods that implementing types must provide.
fn encrypt(&self) -> Self:

This line defines a method signature within the trait.
fn is the keyword to define a function or method.
encrypt is the name of the method.
&self is a method parameter, indicating that this method takes a reference to the instance of the type that implements this trait. This is akin to this in many other languages but is explicitly stated in Rust.
-> Self indicates the return type of this method. 
Self is a special type that refers to the type that implements the trait. This means that the encrypt method will return an instance of the implementing type.
unimplemented!():

Inside the method body, unimplemented!() is a macro call. This macro is a placeholder indicating that the function is not yet implemented. It will cause a panic (a kind of runtime error in Rust) if this method is called. This is useful during development as a temporary placeholder.
The reason for using unimplemented!() here is to provide a default method body. Traits in Rust can provide default implementations for their methods. If a type implements this trait and does not provide its own implementation for encrypt, this default implementation will be used.
*/