#![allow(unused)]
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
/*
一般用法
*/
pub fn open_file() {
  let f = File::open("test.txt");
  let f = match f {
    Ok(file) => file,
    Err(err) => match err.kind() {
      ErrorKind::NotFound => match File::create("test.txt") {
        Ok(fc) => fc,
        Err(fcerr) => {
          panic!("Problem creating the file: {:?}", fcerr)
        }
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error)
      }
    }
  };
}
// .unwrap() 会调用panic! .expect("info") panic! 包含"info"信息，闭包用法
pub fn open_file_nomatch() {
  let f = File::open("test1.txt");
  // 闭包
  f.unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("test1.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error);
    }
  });
}
/* 将错误抛出 */
pub fn open_read_file() -> Result<String, io::Error> {
  let f = File::open("test1.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => { return Err(e) }
  };

  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => return Err(e)
  }
}

// ? 简化异常抛出流程
pub fn read_username_file() -> Result<String, io::Error> {
  let mut f = File::open("file.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  return Ok(s);
}

// ? 简化操作，链式调用 
pub fn read_username_with() -> Result<String, io::Error> {
  let mut s = String::new();
  let mut f = File::open("file.txt")?.read_to_string(&mut s)?;
  return Ok(s);
}
// 读取文件为字符串
pub fn read_file() -> Result<String, io::Error> {
  fs::read_to_string("file.ss.rs")
}