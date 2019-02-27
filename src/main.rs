fn main() {
  let args: Vec<String> = std::env::args().collect();
  let response = match args.len() {
    2 => args[1].trim(),
    _ => "noname"
  };

  println!("Hello {}", response);
}
