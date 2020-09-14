// 函数签名中的生命周期注解
// &'a 显式生命周期的引用
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}