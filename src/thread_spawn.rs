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
  
  handle.join().unwrap(); // 调用join确保创建新线程执行完成，此后逻辑需要等到新线程创建并执行完成后才会执行
}
/**
 * 消息传递并发
 */
pub fn test_mpsc() {
  // 创建消息通道，tx 发送者， rx 接受者
  let (tx, rx) = mpsc::channel();
  // 创建线程使用tx发送消息
  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
  });
  // 使用rx接收消息
  let received = rx.recv().unwrap(); //recv 会阻塞当前主线程，直到接收到一个值，返回 Result<T, E>   try_recv不会阻塞，立刻返回一个Result<T, E>
  println!("Got: {}", received);
}
/**
 * 发送多个消息
 * 通过clone创建多个生产者
 */
pub fn test_for_mpsc() {
  let (tx, rx) = mpsc::channel();
  // clone 生产者
  let tx1 = tx.clone();
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx1.send(val).unwrap(); // 使用clone生产者
      thread::sleep(Duration::from_secs(1));
    } 
  });

  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    } 
  });

  for received in rx { // rx 会等到通道关闭后进行迭代
    println!("Got: {}", received);
  }
}