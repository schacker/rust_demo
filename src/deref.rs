// 当我们将特定类型的值的引用作为参数传递给函数或方法，但是被传递的值的引用与函数或方法中定义的参数类型不匹配时，会发生解引用强制多态。
// 这时会有一系列的 deref 方法被调用，把我们提供的参数类型转换成函数或方法需要的参数类型。
use std::ops::Deref

#[stable(feature = "rust1", since = "1.0.0")]
impl Deref for String {
  type Target = str

  #[inline]
  fn deref(&self) -> &str {
    unsafe { str::from_utf8_unchecked(&self.vec) }
  }
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}

fn test_deref() {
  let m = MyBox::new(String::from("Rust"));
  // 解引用强制多态特性，使得MyBox<String>的值的引用，能将&MyBox<String>变成&String，标准库中String上实现了Deref实现，返回字符串slice，可以将&String变成&str
  hello(&m);
}