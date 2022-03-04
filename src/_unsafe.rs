// 在unsafe块中解引用裸指针
// 裸指针一般在调用C代码接口中使用
// 通过裸指针，就能够同时创建同一地址的可变指针和不可变指针
pub fn test_unsafe() {
  let mut num = 5;
  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
  }
}