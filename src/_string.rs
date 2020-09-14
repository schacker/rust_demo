// Rust 并不允许使用index操作String类型，因为内存存储结构的原因
// 当用户看到字符串时，在内部存储不总是与之index一一对应
// String::from("Здравствуйте").len() Rust的回答是  24，每个 Unicode 标量值需要两个字节存储
// 最后一个 Rust 不允许使用索引获取 String 字符的原因是，索引操作预期总是需要常数时间 (O(1))。但是
// 对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符。
#![allow(unused)]
pub fn test_all() {
  test_string();
  test_string1();
  test_string_add_and_format();
  test_for_string();
}

fn test_string() {
  let mut s = String::from("first string"); // 创建String类型
  s.push_str("add string");
  println!("{}", s);

  let s1 = "from string.to_string()";
  let s2 = s1.to_string(); // 与String::from() 等价
}

fn test_string1() {
  let mut s1 = String::from("foo");
  let s2 = "bar"; //s2 没有所有权，s2为字符串字面量
  s1.push_str(s2); // 传入字符串slice 引用
  println!("s2 is {}", s2);
}
fn test_string_add_and_format() {
  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  // 这里的 + 操作实际上是调用的 add 签名函数，看起来是这样的
  // fn add(self, s: &str) -> String {}，标准库中使用的是泛型，这里由String调用，所以变成&str
  // 但我们的 s2是String类型，为什么能与&str匹配，因为 &String 可强转为 &str，解引用强制多态 &s2 -> &s2[..]
  // 同时add函数签名中获取了s1的所有权

  let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用，+操作必须操作

  println!("{}, {}", s3, s2);
  // format! 宏 不会持有传入变量的所有权
  let sformat = format!("{}-{}", s3, String::from("format test"));
  println!("{}, {}", sformat, s3);
}
fn test_for_string() {
  for c in "नमस्ते".chars() {
    println!("{}", c);
  }
  for c in "नमस्ते".bytes() {
    println!("{}", c);
  }
}
