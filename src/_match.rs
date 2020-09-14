// match 模式  类比其他语言的 switch case
#![allow(unused)]
#[derive(Debug)]
enum State {
  One,
  Thx
}
#[derive(Debug)]
enum Coin {
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven(State)
}
fn test_match(coin: &Coin) -> u8 {
  match coin {
    Coin::One => 1,
    Coin::Two => 2,
    Coin::Three => 4,
    Coin::Four => 8,
    Coin::Five => 16,
    Coin::Six => 32,
    Coin::Seven(state) => {
      println!("{:?}", state);
      64
    }
  }
}
// 匹配是穷尽的，增加了安全性
fn test_match_some_none(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i+1),
    _ => None // 兜底
  }
}
fn test_if_let(coin: Coin) {
  if let Coin::Seven(state) = coin {
    println!("{:?}", state)
  } else {
    println!("{:?}", coin)
  }
}

pub fn test() {
  let coin = Coin::Four;
  let coin7 = Coin::Seven(State::Thx);
  let result = test_match(&coin);
  let result7 = test_match(&coin7);
  println!("{}", result);
  test_if_let(coin7);

  let some = Some(5);
  let six = test_match_some_none(some);
  let none = test_match_some_none(None);
  println!("{:?}, {:?}", six, none);
}