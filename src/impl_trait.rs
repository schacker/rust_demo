pub trait Draw {
  fn draw(&self);
}

pub struct Screen<T: Draw> {
  pub components: Vec<Box<T>>
}

impl<T> Screen<T> 
  where T: Draw {
  pub fn run(&self) {
    for com in self.components.iter() {
      com.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!("i am Button");
  }
}
pub struct SelectBox {
  pub width: u32,
  pub height: u32,
  pub options: Vec<String>,
}
impl Draw for SelectBox {
  fn draw(&self) {
    println!("i am SelectBox"); 
  }
}

/**
 * impl trait 示例
 */
pub fn test_impl_trait() {
  let screen = Screen {
    components: vec![
      Box::new(Button {
        width: 75,
        height: 10,
        label: String::from("OK")
      }),
      // 此种类型为啥不行？这里都使用的是Box::new  可能判断的是内部类型？
      // Box::new(SelectBox {
      //   width: 75,
      //   height: 10,
      //   options: vec![String::from("OK")]
      // })
    ],
  };

  screen.run();
}