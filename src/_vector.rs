#![allow(unused)]
#[allow(dead_code)]

pub fn test_all() {
  test_vector();
  test_vector1();
  test_vector2();
  test_for_vector();
  test_enum_vector();
}

fn test_vector() {
  let mut vec1 = Vec::new();
  // 泛型（类型注解）
  let vector: Vec<i32> = Vec::new();
  // 初始值
  let mut vec = vec![1,2,3]; // vec! 向量宏
  vec.push(1);
  vec1.push(2);
}

fn test_vector1() {
  let mut vec = vec![1,23,4,45];
  let third = &vec[2];

  match vec.get(2) {
    Some(third) => {
      println!("{}", third);
    }
    None => println!("没找到对应的元素")
  }
}

fn test_vector2() {
  let mut vec = vec![1,2,3];

  // let not = &vec[10]; //直接引用超出索引的集合元素 导致程序panic
  let not1 = vec.get(2); // 返回Option<T> Some or None

  match not1 {
    Some(&i32) => println!("{:?}", not1),
    None => println!("没找到对应元素")
  }
}

fn test_for_vector() {
  let mut vec = vec![1,2,3];
  println!("{}", "遍历向量集合");
  // 必须使用引用，for loop 等操作同样遵循所有权和借用规则
  for i in &mut vec {
    *i += 100; //* 解引用操作
    println!("{}", i);
  }
  for mut i in vec {
    println!("{}", i);
    i += 100;
    println!("{}", i);
  }

  // for mut i in &mut vec {
  //   println!("{}", i);
  //   *i += 100;
  //   println!("{}", i);
  // }
}
#[derive(Debug)]
enum Vector {
  Int(i32),
  Float(f64),
  Text(String)
}
fn test_enum_vector() {
  let mut row = vec![
    Vector::Int(3),
    Vector::Text(String::from("blue")),
    Vector::Float(10.12),
  ];
  for i in &row {
    println!("{:?}", i);
  }
  row.pop();
  for i in &row {
    println!("{:?}", i);
  }
}