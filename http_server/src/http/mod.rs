pub use request::Request;
pub use method::Method; 
pub use request::ParseError;

//Specify public interface for module
pub mod method;
pub mod request;
pub mod query_string;