#![allow(unused)]
// 单独的Message枚举
pub enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32)
}

impl Message {
  fn call(&self) -> bool {
    println!("{}", "call done");
    return true
  }
}

pub fn test_message() {
  let message = Message::Write(String::from("test message"));
  message.call();
}

struct QuitMessage;
struct MoveMessage {
  x: i32,
  y: i32
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);