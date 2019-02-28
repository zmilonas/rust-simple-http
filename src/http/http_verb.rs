use std::io::Error;
use std::io::ErrorKind;

#[derive(Debug)]
pub enum Verb {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
impl Verb {
    pub fn from_string(verb: &str) -> Result<self::Verb, Error> {
        match verb {
            "GET" => Ok(self::Verb::GET),
            "HEAD" => Ok(self::Verb::HEAD),
            "POST" => Ok(self::Verb::POST),
            "PUT" => Ok(self::Verb::PUT),
            "DELETE" => Ok(self::Verb::DELETE),
            "CONNECT" => Ok(self::Verb::CONNECT),
            "OPTIONS" => Ok(self::Verb::OPTIONS),
            "TRACE" => Ok(self::Verb::TRACE),
            "PATCH" => Ok(self::Verb::PATCH),
            _ => Err(Error::new(ErrorKind::InvalidInput, format!("{} not found in http_verb", verb))),
        }
    }
}
