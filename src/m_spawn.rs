use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn create_thread() {
  let v = vec![1,2,3];

  let handle = thread::spawn(move || { // move 获取v的所有权
    for _i in 1..10 {
      println!("Here's a vector: {:?}", v);
      thread::sleep(Duration::from_millis(1));
    }
  });

  
  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
  
  handle.join().unwrap(); // 调用join确保创建新线程执行完成
}

pub fn test_mpsc() {
  let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap(); //recv 会阻塞当前主线程，直到接收到一个值，返回 Result<T, E>   try_recv不会阻塞，立刻返回一个Result<T, E>
    println!("Got: {}", received);
}