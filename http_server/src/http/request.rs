// use super::method::Method; // use the method module in the parent module
use super::method::{MethodError, Method}; // use the method module in the parent module
use std::convert::TryFrom; // Simple and safe type conversions that may fail in a controlled way under some circumstances.
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//use std::error::Error;
use std:: fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

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

// using try from for cases when it does not return a request option from the enum
// because try from is a returns a result
// conversion might fail.
impl TryFrom<&[u8]> for Request { 
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?
   
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
      
   

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        // override method 
        let method: Method = method.parse()?;

        //define query string
        let mut query_string = None;
        // find method for string slice
        // returns an option
        if let Some(i) = path.find('?'){
            query_string = Some(&path[i + 1..]);
                //reassign path to be everything before ?
            path = &path[..i];

        }
        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // to get the rest of the string we add 1 to the index of what we've returned
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
