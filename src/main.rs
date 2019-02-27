use std::io::{self, BufRead};
use std::collections::HashMap;
use std::process;
use std::io::Error;
use std::io::ErrorKind;
use RequestPart::TYPE;
use RequestPart::HEADERS;
use RequestPart::BODY;

#[derive(Debug)]
enum HttpVerb {
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

#[derive(Debug)]
struct HttpRequest {
  verb: HttpVerb,
  path: String,
  proto: String,
  headers: HashMap<String, String>,
  body: String,
}

impl HttpVerb {
  fn from_string(verb: &str) -> Result<HttpVerb, Error> {
    match verb {
      "GET" => Ok(HttpVerb::GET),
      "HEAD" => Ok(HttpVerb::HEAD),
      "POST" => Ok(HttpVerb::POST),
      "PUT" => Ok(HttpVerb::PUT),
      "DELETE" => Ok(HttpVerb::DELETE),
      "CONNECT" => Ok(HttpVerb::CONNECT),
      "OPTIONS" => Ok(HttpVerb::OPTIONS),
      "TRACE" => Ok(HttpVerb::TRACE),
      "PATCH" => Ok(HttpVerb::PATCH),
      _ => Err(Error::new(ErrorKind::InvalidInput, format!("{} not found in HttpVerb", verb))),
    }
  }
}

#[derive(Debug, PartialEq)]
enum RequestPart {
  TYPE,
  HEADERS,
  BODY,
}

fn run() -> Result<HttpRequest, Error> {
  let stdin = io::stdin();
  let lines = stdin.lock().lines();
  let mut http_request: HttpRequest = HttpRequest {
    verb: HttpVerb::GET,
    path: "".to_string(),
    proto: "".to_string(),
    headers: HashMap::new(),
    body: "".to_string()
  };
  let mut line_count: u64 = 1;
  let mut request_part = RequestPart::TYPE;
  for line_result in lines {
    match line_result {
      Ok(line) => {
        if line_count == 1 {
          request_part = TYPE;
        } else if line_count > 1 && request_part != BODY {
          request_part = HEADERS;
          if line == "" {
            request_part = BODY;
          }
        }
        line_count += 1;

        match request_part {
          TYPE => {
            let l: Vec<&str> = line.split(' ').collect();
            http_request.verb = match HttpVerb::from_string(l[0]) {
              Ok(verb) => verb,
              Err(e) => return Err(e),
            };
            http_request.path = l[1].to_string();
            http_request.proto = l[2].to_string();
          },
          HEADERS => {
            let l: Vec<&str> = line.split(": ").collect();
            http_request.headers.insert(l[0].to_string(), l[1].to_string());
          },
          BODY => {
            http_request.body.push_str(line.trim());
          },
        }
      }
      Err(e) => return Err(e),
    }
  }

  return Ok(http_request);
}

fn main() {

  let result = run();

  if let Err(e) = result {
    eprintln!("Application error: {}", e);

    process::exit(1);
  }
  println!("{:?}", result.unwrap());

}
