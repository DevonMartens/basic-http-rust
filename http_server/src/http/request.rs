// use super::method::Method; // use the method module in the parent module
use super::method::Method; // use the method module in the parent module

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