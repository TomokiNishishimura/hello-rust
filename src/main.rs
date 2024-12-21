fn main() {
  let mut message = "Hello, Wasm!";
  println!("変数messageに束縛された値: {}", message);
  message = "Hello, World!";
  println!("変数messageに束縛された値: {}", message);
}
