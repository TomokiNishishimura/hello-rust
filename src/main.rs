fn main() {
  let mut message = "Hello, Wasm!";
  println!("変数messageに束縛された値: {}", message);
  message = "Hello, World!";
  println!("変数messageに束縛された値: {}", message);
  greeting();
  greeting2();
}

// u32型の値を返す関数
// u32型は符号なし32ビット整数型
fn greeting() -> u32 {
  println!("Hello, from WebAssembly!");
  42
}

// ユニット型を返す関数
// ユニット型は意味を持たない値を表す型
// 返り値がない場合に使われる
// 省略可能
fn greeting2() -> () {
  println!("Hello, from WebAssembly!");
}
