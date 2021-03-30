extern crate wasm_bindgen;
extern crate console_error_panic_hook;

use regex::{Regex, Captures};
use itertools::Itertools;

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use std::panic;

// rust中调用js函数
#[wasm_bindgen]
extern {
	pub fn alert(s: &str);
}
// 能在js中调用的rust函数
#[wasm_bindgen]
pub fn greet(name: &str) {
	alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn isAppWebView(userAgent: &str, checkType: &str) -> bool {
  console_error_panic_hook::set_once();
  match checkType {
    "link" => return isLink(userAgent),
    "lianjia" => return isLianjia(userAgent),
    "deyou" => return isDeyou(userAgent),
    "baichuan" => return isBaichuan(userAgent),
    "atom" => return isAtom(userAgent),
    "linkxinfang" => return isLinkXinfang(userAgent),
    "vrstudio" => return isVRStudio(userAgent),
    _ => return isLink(userAgent) || 
      isLianjia(userAgent) || 
      isDeyou(userAgent) || 
      isBaichuan(userAgent) || 
      isAtom(userAgent) || 
      isLinkXinfang(userAgent) ||
      isVRStudio(userAgent)
  }
}

fn isLink(userAgent: &str) -> bool {
  let r = Regex::new(r"(?i)Lianjia/(?i)Home(?i)Link/([0-9a-z\.\-]+)").unwrap();
  return r.is_match(userAgent)
}
fn isVRStudio(userAgent: &str) -> bool {
  let r = Regex::new(r"(?i)V(?i)R(?i)Studio\s*([0-9a-z\.\-]+)").unwrap();
  return r.is_match(userAgent)
}
fn isLinkXinfang(userAgent: &str) -> bool {
  let r = Regex::new(r"(?i)Home(?i)Link/([0-9a-z\.\-]+)").unwrap();
  return r.is_match(userAgent)
}
fn isDeyou(userAgent: &str) -> bool {
  let r = Regex::new(r"(?i)Lianjia/(?i)Alliance/?([0-9a-z\.\-]+)").unwrap();
  let r1 = Regex::new(r"(?i)Lianjia(?i)Alliance/?([0-9a-z\.\-]+)").unwrap();
  return r.is_match(userAgent) || r1.is_match(userAgent)
}
fn isBaichuan(userAgent: &str) -> bool {
  let r = Regex::new(r"(?i)Lianjia/(?i)lianjiabaichuan/([0-9a-z\.\-]+)").unwrap();
  return r.is_match(userAgent)
}
fn isBeike(userAgent: &str) -> bool {
  let r = Regex::new(r"(?i)lianjiabeike/([0-9a-z\.\-]+)").unwrap();
  return r.is_match(userAgent)
}
fn isAtom(userAgent: &str) -> bool {
  let r = Regex::new(r"(?i)lianjiaatom/([0-9a-z\.\-]+)").unwrap();
  return r.is_match(userAgent)
}
fn isLianjia(userAgent: &str) -> bool {
  let r = Regex::new(r"(?i)Lianjia").unwrap();
  if (r.is_match(userAgent)) {
    return !(
      isDeyou(userAgent) || 
      isBaichuan(userAgent) || 
      isBeike(userAgent) ||
      isLink(userAgent) ||
      isAtom(userAgent)
    )
  }
  return false
}

#[wasm_bindgen]
pub fn facilities(n: i32) -> i32 {
  if n <= 0 {
    return 1;
  }
  return n * facilities(n - 1);
}

#[wasm_bindgen]
pub fn appendSearch(uri: &str, search: &str) -> String {

  let mut _uri: Option<&str> = Some(uri);
  if !uri.is_empty() && (uri.ends_with("?") || uri.ends_with("&")) {

    _uri = uri.get(0..(uri.len()-1));
  }

  let sFirst = search.starts_with("?") || search.starts_with("&");

  let mut _search: Option<&str> = Some(search);
  if sFirst {
    _search = search.get(1..(search.len()));
  }

  // let mut m_uri: &str;
  match _search {
    Some(s) => {
      if s.len() == 0 { 
        match _uri {
          Some(_s) => {
            return format!("{}", _s);
          },
          None => {
            
          }
        }
      }

    },
    None => {
            
    }
  };

  let spliter = match uri.contains("?") {
    true => "&",
    false => "?"
  };

  return format!("{}{}{}", _uri.unwrap_or_default(), spliter, _search.unwrap_or_default())
}
#[wasm_bindgen]
pub fn evalDOM() {
  
}