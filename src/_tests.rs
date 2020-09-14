// cargo test -- --nocapture //不捕获测试输出
// cargo test -- --ignore //忽略包含#[ignore]注解测试用例
#[cfg(test)] // cargo build 阶段不会包含 #[cfg(test)] 标注模块
pub mod tests {
    #[test]
    // #[should_panic] //执行成功则panic
    // #[ignore] //忽略该条测试用例
    pub fn it_works() {
        assert_eq!(2 + 2, 4);
        // assert_ne! 不等于断言
        // assert 布尔断言
        println!("something, {}", String::from("sssssssssss"))
    }
    #[test]
    pub fn fine() {
      
    }
    
}