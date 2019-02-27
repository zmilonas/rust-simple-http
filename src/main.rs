fn main() {
  let args: Vec<String> = std::env::args().collect();
  let response: String = match args.len() {
    2 => args[1].to_string(),
    _ => String::from("noname")
  };

  println!("Hello {}", response);
}

