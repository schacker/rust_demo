use std::mem::drop;
// drop trait 变量或实例的值离开作用域时执行的代码
// 当实例离开作用域 Rust 会自动调用 drop，并调用我们指定的代码。变量以被创建时相反的顺序被丢弃，所以 d 在 c 之前被丢弃
#[derive(Debug)]
struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data `{}`!\n", self.data);
  }
}
// 使用Drop trait清理
pub fn run() {
  let c = CustomSmartPointer { data: String::from("my stuff") };
  let d = CustomSmartPointer { data: String::from("other stuff") };
  println!("CustomSmartPointers created. {:?}", c);
}
// 调用标准库中 std::mem::drop 提前释放变量
pub fn run_drop() {
  let c = CustomSmartPointer { data: String::from("my run_drop") };
  println!("CustomSmartPointers created. {:?}", c);
  drop(c);
  println!("CustomSmartPointer dropped before the end of main.");
}
