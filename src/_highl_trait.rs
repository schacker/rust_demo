use std::fmt;

// 完全限定语法 <Type as Trait>::function(receiver_if_method, next_arg, ...);
trait Pilot {
  fn fly(&self);
}
trait Wizard {
  fn fly(&self);
}
struct Human;

impl Pilot for Human {
  fn fly(&self) {
    println!("This is your captain speaking.");
  }
}

impl Wizard for Human {
  fn fly(&self) {
    println!("Up!");
  }
}
impl Human {
  fn fly(&self) {
    println!("*waving arms furiously*");
  }
}

// 消除歧义的高级trait
pub fn test_highl_trait() {
  let person = Human;
  Pilot::fly(&person);
  Wizard::fly(&person);
  person.fly();
}


struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}]", self.0.join(","))
  }
}

// Display 的实现使用 self.0 来访问其内部的 Vec<T>，因为 Wrapper 是元组结构体而 Vec<T> 是结构体总位于索引 0 的项。接着就可以使用 Wrapper 中 Display 的功能了。
pub fn test_wrapper() {
  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);
}