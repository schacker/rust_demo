mod test {
  pub fn testin() {
    println!("{}", "testin")
  }
}
fn ones() {
  println!("{}", "super ones");
  test::testin();
}
#[allow(unused)]
pub mod one {
  pub fn one_fn() {
    super::ones();
  }
  pub mod two_one {
    pub fn two_one_fn() {
      super::super::one::one_fn();
      super::super::ones();
      super::two_two::three::three_one_fn();
      inner();
    }
    fn inner() {
      println!("{}", "mod inner");
    }
  }
  pub mod two_two {
    pub mod three {
      pub fn three_one_fn() {
        println!("{}", "通过super调用");
      }
    }
  }
}