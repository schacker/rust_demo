use hello_macro::HelloMacro;
struct Pancakes;

impl HelloMacro for Pancakes {
  fn hello_macro() {
    println!("Hello, Macro! My name is Pancakes!");
  }
}

pub fn t_macro() {
  Pancakes::hello_macro();
}