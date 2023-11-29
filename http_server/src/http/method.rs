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
    type Err = String;

    // match on string
    match s {
        // if string is X return wrapped req
        "GET" => Ok(Self::GET),
        "DELETE",
        "POST",
        "PUT",
        "HEAD",
        "CONNECT",
        "OPTIONS",
        "TRACE",
        "PATCH",
    }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}