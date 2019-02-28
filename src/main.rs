use std::io::{self, BufRead};
use std::collections::HashMap;
use std::process;
use std::io::Error;

mod http;
use http::http_verb;

#[derive(Debug)]
struct HttpRequest {
  verb: http_verb::Verb,
  path: String,
  proto: String,
  headers: HashMap<String, String>,
  body: String,
}

#[derive(Debug, PartialEq)]
enum HttpRequestRawBodyPart {
  TYPE,
  HEADERS,
  BODY,
}

fn run() -> Result<HttpRequest, Error> {
  let stdin = io::stdin();
  let lines = stdin.lock().lines();
  let mut http_request: HttpRequest = HttpRequest {
    verb: http_verb::Verb::CONNECT,
    path: "".to_string(),
    proto: "".to_string(),
    headers: HashMap::new(),
    body: "".to_string()
  };
  let mut line_count: u64 = 1;
  let mut request_part = HttpRequestRawBodyPart::TYPE;
  for line_result in lines {
    match line_result {
      Ok(line) => {
        if line_count == 1 {
          request_part = HttpRequestRawBodyPart::TYPE;
        } else if line_count > 1 && request_part != HttpRequestRawBodyPart::BODY {
          request_part = HttpRequestRawBodyPart::HEADERS;
          if line == "" {
            request_part = HttpRequestRawBodyPart::BODY;
          }
        }
        line_count += 1;

        match request_part {
          HttpRequestRawBodyPart::TYPE => {
            let l: Vec<&str> = line.split(' ').collect();
            http_request.verb = match http_verb::Verb::from_string(l[0]) {
              Ok(verb) => verb,
              Err(e) => return Err(e),
            };
            http_request.path = l[1].to_string();
            http_request.proto = l[2].to_string();
          },
          HttpRequestRawBodyPart::HEADERS => {
            let l: Vec<&str> = line.split(": ").collect();
            http_request.headers.insert(l[0].to_string(), l[1].to_string());
          },
          HttpRequestRawBodyPart::BODY => {
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
