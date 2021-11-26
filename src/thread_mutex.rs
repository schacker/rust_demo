use std::sync::{Mutex,Arc};
use std::thread;

pub fn test_mutex() {
  let m = Mutex::new(5);

  {
    let mut num = m.lock().unwrap();
    *num = 6
  }

  println!("m = {:?}", m);
}

pub fn test_multi_mutex() {
  // 原子引用计数，多线程之间共享所有权必须使用，Rc<T>引用计数在此不能使用，因为Rc<T>无法保证线程安全，但Arc<T>带有性能惩罚，所以在必要的时候才会使用原子引用计数
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
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