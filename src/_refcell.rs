
#![allow(unused)]
fn test_refcell() {
  /**
   * trait 类似接口
   */
  pub trait Messenger {
    fn send(&self, msg: &str);
  }
  /**
   * 结构体
   */
  pub struct LimitTracker<'a, T: Messenger> {
      messenger: &'a T,
      value: usize,
      max: usize,
  }
  /**
   * 实现结构体
   * - 限定 T 的类型
   */
  impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
      LimitTracker {
          messenger,
          value: 0,
          max,
      }
    }

    pub fn set_value(&mut self, value: usize) {
      self.value = value;

      let percentage_of_max = self.value as f64 / self.max as f64;

      if percentage_of_max >= 1.0 {
        self.messenger.send("Error: You are over your quota!");
      } else if percentage_of_max >= 0.9 {
        self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
      } else if percentage_of_max >= 0.75 {
        self.messenger.send("Warning: You've used up over 75% of your quota!");
      }
    }
  }
}
