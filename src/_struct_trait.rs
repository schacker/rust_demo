#![allow(unused)]
pub struct Demo<T> {
  x: T,
  y: T
}

impl<T> Demo<T> {
  fn x(&self) -> &T{
    &self.x
  }
}
impl Demo<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}

pub fn test() {
  let demo = Demo {
    x: 1, y: 2
  };
  let p1 = Point { x: 5, y: 10.4 };
  let p2 = Point { x: "Hello", y: 'c'};

  let p3 = p1.mixup(p2);
  
  println!("{}, {:?}", demo.x(), p3)
}