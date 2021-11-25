use std::ops::Deref;
/**
 * 最简单直接的智能指针是 box，其类型是 Box<T>。 box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。
 * 除了数据被储存在堆上而不是栈上之外，box 没有性能损失。不过也没有很多额外的功能。它们多用于如下场景：
 * 
 * - 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
 * - 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
 * - 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候
 * 
 * 我们会在 “box 允许创建递归类型” 部分展示第一种场景。在第二种情况中，转移大量数据的所有权可能会花费很长的时间，因为数据在栈上进行了拷贝。
 * 为了改善这种情况下的性能，可以通过 box 将这些数据储存在堆上。接着，只有少量的指针数据在栈上被拷贝。
 * 第三种情况被称为 trait 对象（trait object），第十七章刚好有一整个部分 “为使用不同类型的值而设计的 trait 对象” 专门讲解这个主题。
 */
pub fn test_box() {
  let x = 5;
  let b = MyBox::new(x);
  println!("b = {:?}", *b); // 实现了Deref的解引用，等同于 *(b.deref())
}
// 包含一个元素的元祖结构体，泛型参数T
struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}
// 实现解引用功能
impl<T> Deref for MyBox<T> {
  // 定义用于此trait的关联类型
  type Target = T;
  // 实现方法
  fn deref(&self) -> &T {
    &self.0 // 返回值的引用
  }
}

// struct SomeBox<T, M>(T, M);
// impl<T, M> SomeBox<T, M> {
//   fn new(a: T, b: M) -> SomeBox<T, M> {
//     SomeBox(a, b)
//   }
// }

// impl<T, M> Deref for SomeBox<T, M> {
//   type Target = (T, M);

//   fn deref(&self) -> (&T, &M) {
//     (&self.0, &self.1)
//   }
// }