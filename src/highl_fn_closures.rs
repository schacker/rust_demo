fn add_one(x: i32) -> i32 {
  x+1
}

/**
 * 高级函数，以函数为参数
 */
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}

pub fn test_highl_fn() {
  let answer = do_twice(add_one, 5);

  println!("The answer is: {}", answer);
}