fn main() {
  let mut args: Vec<String> = std::env::args().collect();

  println!("Hello {}", args[1]);
}
