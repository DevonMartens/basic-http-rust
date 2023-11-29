use std::str::FromStr;
// Remember 9 methods in Request_Type.md
pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

// string that contrains method from request
impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
    // match on string
    match s {
        // if string is X return wrapped req
        "GET" => Ok(Self::GET),
        "DELETE" => Ok(Self::DELETE),
        "POST" => Ok(Self::POST),
        "PUT" => Ok(Self::PUT),
        "HEAD" => Ok(Self::HEAD),
        "CONNECT" => Ok(Self::CONNECT),
        "OPTIONS" => Ok(Self::OPTIONS),
        "TRACE" => Ok(Self::TRACE),
        "PATCH" => Ok(Self::PATCH),
        _ => Err(MethodError),
    }
    }
}

// custom error

pub struct MethodError;