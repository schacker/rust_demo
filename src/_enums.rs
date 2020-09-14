#![allow(unused)]
#[derive(Debug)]
pub enum IpAddr {
  V4(u32, u32, u32, u32),
  V6(String)
}

pub fn gen_ip() {
  let v4 = IpAddr::V4(127, 0, 0, 1);
  let v6 = IpAddr::V6(String::from("::1"));

  println!("{:?}, {:?}", v4, v6)
}
/**
 * 空值 泛型枚举
 **/
pub enum Option<T> {
  Some(T),
  None
}
pub enum Result<T, E> {
  Ok(T),
  Err(E)
}