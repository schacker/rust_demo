use std::sync::{Mutex,Arc};
use std::thread;

pub fn test_mutex() {
  let m = Mutex::new(5);

  {
    // num为MutexGuard智能指针，实现了Deref来指向内部数据，当离开作用域后，自动释放锁
    let mut num = m.lock().unwrap();
    *num = 6
  }
  // 这里已经释放了m的锁
  println!("m = {:?}", m);
}

pub fn test_multi_mutex() {
  // 原子引用计数：多线程之间共享所有权必须使用的特性
  // Rc<T>引用计数在此不能使用，因为Rc<T>无法保证线程安全，但Arc<T>带有性能惩罚，所以在必要的时候才会使用原子引用计数
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    // 每个线程中使用的都是clone于counter，原子引用计数
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();

      *num += 1;
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
  println!("Result: {}", *counter.lock().unwrap());
}