#![allow(unused)]
// HashMap 键-值 键同类型  值同类型
use std::collections::HashMap;

pub fn test_all() {
  test_hashmap();
  test_for_hashmap();
  test_update();
  test_for_update();
}

fn test_hashmap() {
  let mut map = HashMap::new();

  map.insert(String::from("first"), 123);
  map.insert(String::from("second"), 1232);
  // 向量
  let teams  = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];
  // _ 占位符  类型注解
  let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}

fn test_for_hashmap() {
  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }
}
fn test_update() {
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  // entry 判断是否有对应参数的关联只
  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{:?}", scores);
}

fn test_for_update() {
  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0); // or_insert() 返回对应对应值的可变引用
    *count += 1;
  }

  println!("{:?}", map);
}