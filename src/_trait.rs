#![allow(unused)]
use std::cmp::PartialOrd;

fn largest(list: &[i32]) -> i32 {
  let mut max = list[0];

  for &item in list {
    if max < item {
      max = item
    }
  }

  max
}
/**
 * - 泛型
 * - 通过 + 指定多个 trait bound
 */
fn largest_trait<T: PartialOrd + Copy >(list: &[T]) -> T {
  let mut max = list[0];

  for &item in list.iter() {
    if max < item {
      max = item
    }
  }
  max
}
// 使用where简化trait bounds
fn largest_trait_where<T>(list: &[T]) -> T 
  where T: PartialOrd + Copy 
{
  let mut max = list[0];

  for &item in list.iter() {
    if max < item {
      max = item
    }
  }
  max
}
pub fn test_largest() {
  let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_trait(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_trait(&char_list);
    println!("The largest char is {}", result);
}