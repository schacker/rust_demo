#![allow(unused)]
#[allow(dead_code)]
#[allow(unused_imports)]
use std::io::Result as IoResult; // 避免重复模块名称冲突
use std::fmt::Result;
use std::{ cmp::Ordering, io::copy};
use std::collections:: { self };
#[allow(unused)]
pub fn test() -> IoResult<String> {
  let mut buf = String::new();
  buf.push_str("sssss");
  Ok(buf)
}