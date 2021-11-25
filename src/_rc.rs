pub enum List {
  Cons(i32, Rc<List>),
  Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

/**
 * 需要使用 use 语句将 Rc<T> 引入作用域，因为它不在 prelude 中。
 * 创建了存放 5 和 10 的列表并将其存放在 a 的新的 Rc<List> 中。
 * 接着当创建 b 和 c 时，调用 Rc::clone 函数并传递 a 中 Rc<List> 的引用作为参数。
 * 也可以调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用 Rc::clone。
 * Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。Rc::clone 只会增加引用计数，这并不会花费多少时间。
 * 深拷贝可能会花费很长时间。
 * 通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。
 * 当查找代码中的性能问题时，只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。
 */
pub fn test_rc() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));
  let b = Cons(3, Rc::clone(&a)); // clone增加引用计数，不会做数据深拷贝
  println!("count after creating b = {}", Rc::strong_count(&a)); // 强引用计数
  {
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));

}