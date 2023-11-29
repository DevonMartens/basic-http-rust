// Import necessary modules and traits from the standard library and other places.
use super::method::{Method, MethodError}; // Import Method and MethodError from a parent module.
use crate::http::query_string::QueryString; // Import QueryString from a parent module.
use std::convert::TryFrom; // Import TryFrom trait for type conversion.
//use std::error::Error; // Import Error trait for error handling.
use std::fmt::{Debug, Display, Formatter, Result as FmtResult}; // Import formatting traits and types.
use std::str; // Import string handling utilities.
use std::str::Utf8Error; // Import Utf8Error for handling UTF-8 encoding errors.

// GET search?name=abc&sort=1 HTTP/1.1\r\r...HEADERS...

// Define a Request struct which holds references to parts of a string buffer.
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

// Implementation block for Request.
impl<'buf> Request<'buf> {
    // Getter method for the path.
    pub fn path(&self) -> &str {
        &self.path
    }

    // Getter method for the method.
    pub fn method(&self) -> &Method {
        &self.method
    }

    // Getter method for the query_string.
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

// Implement the TryFrom trait for Request to convert from a byte slice.
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // Main function to try converting a byte slice to a Request.
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?; // Convert buffer to string slice, handling potential UTF-8 errors.

        // Parse the HTTP method, path, and protocol.
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        // Validate the protocol.
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?; // Parse the method, handling potential method parsing errors.

        // Parse query string if present.
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        // Construct and return the Request object.
        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

// Function to get the next word (or token) in a request string.
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

// Define an enumeration for different types of parsing errors.
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

// Implementations for ParseError.
impl ParseError {
    // Method to get a human-readable message for each error type.
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

// Implement conversion from MethodError to ParseError.
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

// Implement conversion from Utf8Error to ParseError.
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

// Implement Display trait for ParseError for better error messages.
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// Implement Debug trait for ParseError to enable debugging capabilities.
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
   
}
}