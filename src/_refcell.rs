#![allow(unused)]
use std::cell::RefCell;
use std::rc::{Rc, Weak};

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
/**
 * 我们希望能够 Node 拥有其子节点，同时也希望通过变量来共享所有权，以便可以直接访问树中的每一个 Node，
 * 为此 Vec<T> 的项的类型被定义为 Rc<Node>。我们还希望能修改其他节点的子节点，
 * 所以 children 中 Vec<Rc<Node>> 被放进了 RefCell<T>。
 */
#[derive(Debug)]
pub struct Node {
  value: i32,
  parent: RefCell<Weak<Node>>,
  children: RefCell<Vec<Rc<Node>>>
}

pub fn rc_refcell() {
  let leaf =  Rc::new(Node{
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![])
  });
  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

  let branch = Rc::new(Node{
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)])
  });

  *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); 
}