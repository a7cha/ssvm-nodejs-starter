use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  println!("The Rust function say() received {}", s);
  let r = String::from("hello ");
  return r + s;
}

#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
  match n {
    1 => 1,
    2 => 1,
    _ => fib(n - 1) + fib(n - 2)
  }
}
