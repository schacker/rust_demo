mod _struct;
mod _enums;
mod _enum_struct;
mod _option;
mod _match;
mod _mod;
// use crate::_mod::one::two_two;
use _mod::one::two_two;
use std::collections::HashMap;
mod _as;
mod modules; // 查找modules/mod.rs 
mod _vector;
mod _string;
mod _hashmap;
// mod _panic;
mod _file;
mod _trait;
mod _struct_trait;
mod _closures;
mod _all_trait;
mod _iterators;
mod _utils;
mod _log;
mod injection;
mod _box;
mod _rc;
mod _refcell;
mod drop;
mod thread_spawn;
mod thread_mutex;
mod impl_trait;
mod highl_fn_closures;
mod _macro;

fn main() {
    _macro::t_macro();
    return;
    highl_fn_closures::test_highl_fn();
    impl_trait::test_impl_trait();
    thread_mutex::test_multi_mutex();
    thread_mutex::test_mutex();
    thread_spawn::test_for_mpsc();
    thread_spawn::test_mpsc();
    thread_spawn::create_thread();
    _refcell::rc_refcell();
    drop::run();
    drop::run_drop();
    _rc::test_rc();
    _box::test_box();
    // _log::testlog();
    let r = _utils::is_app_webview("vrStudio 10.1", "");
    let f = _utils::facilities(10);
    let s = _utils::append_search("/a/b/c", "?a=1&b=2&c=3");
    // _utils::testlog();
    println!("{}-{}-{}", r, f, s);
    _iterators::test_filter_iterators();
    _iterators::test_iterators();
    _closures::generate_workout(2, 45);
    _closures::simulated_expensive_calculation(2);
    _struct_trait::test();
    _trait::test_largest();
    _all_trait::test_trait();
    _file::open_file();
    _file::open_file_nomatch();
    // _panic::panic();
    // hashmap
    _hashmap::test_all();
    // string
    _string::test_all();
    // vector
    _vector::test_all();
    // 文件mod
    modules::one::one_fn();
    // map
    test_hashmap();
    // mod
    two_two::three::three_one_fn();
    _mod::one::two_two::three::three_one_fn();
    // match
    _match::test();
    // option
    _option::test_option();
    // _enum_struct
    _enum_struct::test_message();
    // enums
    _enums::gen_ip();
    // struct
    let dimen = _struct::Rect {
        width: 12,
        height: 32
    };
    let areas = _struct::area(&dimen);
    println!("结构体：{}", areas);
    println!("{:#?}, {}, {}, \n{:#?}", dimen, dimen.area(), dimen.can_hold(&dimen), _struct::Rect::square(32));
    _struct::structfn();
    return;
    // base
    let mut s = String::from("hello"); // 动态申请内存

    s.push_str(", schacker");

    println!("{}", s);

    let s1: String = takes_and_give_back(s); // s 移动到func函数内部
    println!("交回所有权 {}", s1); // 无法访问s 但takes_and_give_back函数持有s所有权后交回s所有权给s1

    let s2: String = give_ownership(); // 返回新的所有权变量指针
    println!("返回全新变量所有权 {}", s2);

    let (s3, len) = return_tuple(s2);
    println!("函数返回全新元祖 {}, {}", s3, len);
    let len1 = addr(&s3); // 传递引用，不持有所有权，s3所有权仍然在main中
    println!("传递引用，不持有所有权 {}, {}", len1, s3);
    // 同一个变量只能有一个可变引用，避免数据竞争
    let mut modifys = String::from("sss");
    modify_borrowing(&mut modifys);
    // 动态作用域
    dymp_scope();
    dangling();
    let first = String::from("a sa");
    let redirect = "something redirect"; // 字符串字面值 为 &str slice 类型
    first_word(redirect);
    let fi = first_word(&first);
    println!("{}, {}", fi, first);
    // slice 部分引用
    slice();
}
/**
 * - hasmap
 */
fn test_hashmap() {
    let mut hmap = HashMap::new();
    hmap.insert(1, 23);
    hmap.insert(1, 25);
    println!("可变hasmap {:?}", hmap);
}

/**
 * - 手动切片
 */
fn slice() {
    let s = String::from("hello world");
    
    let p1 = &s[0..5]; // slice 功能，p1 只有 s 的部分引用
    let p2 = &s[6..11];

    println!("手动切片 {}, {}", p1, p2)
}
/**
 * - 字节位置，参数类型为字符串 slice 类型
 * - String 可变、堆上、字节缓冲区
 * - str 定长、不可变，解引用为堆引用，字面量为静态内存
 * - &str 字符串切片（借用），不可变，也不放弃所有权？
 * - &String String的借用，指针，可传递，但不放弃所有权，等价于 &str
 * - & 为引用符，获取值但不获取所有权
 * - &mut 可变引用
 * 
 * - String::from 动态、堆上
 * 
 * - 想要一个字符串的只读视图，或者&str作为一个函数的参数，那就首选&str。如果想拥有所有权，想修改字符串那就用String
 * - 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
 * - 引用必须总是有效的。
 */
fn first_word(s: &str) -> &str {
    let bs = s.as_bytes();
    for (i, &item) in bs.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
/**
 * 悬垂引用，编译报错
 * 
 * =>
 * 
 * 返回变量，且交回作用域，即可
 */
fn dangling() -> String {
    let s = String::from("dangling");
    // &s //不能直接返回，s在函数执行结束即释放，所以这里的引用指向的是无效String，造成了悬垂引用
    s
}
/**
 * - 动态作用域
 * - 可变引用必须在不可变引用之后申明
 */
fn dymp_scope() {
    // 可变变量
    let mut s = String::from("dymp_scopr");
    let p1 = &s; // 不可变引用
    let p2 = &s;

    // let p3 = &mut s; //可变引用
    println!("{}, {}", p1, p2); //p1的不可变作用域穿过了p3的可变作用域，因此编译失败
    let p3 = &mut s; //可变引用
    println!("{}", p3);
}
// fn func(s: String) -> String {
//     println!("{}", s);
//     return s;
// }
/**
 * 返回全新变量所有权
 */
fn give_ownership() -> String {
    let s = String::from("ownership");
    return s; //返回并所有权交给调用此函数的作用域（函数）
}
/**
 * 交回传入变量所有权
 * 变量传递
 */
fn takes_and_give_back(s: String) -> String {
    return s; //返回并所有权交给调用此函数的作用域（函数）
}
/**
 * 交回多重数据所有权 元组
 */
fn return_tuple(s: String) -> (String, usize) {
    let len = s.len();
    return (s, len)
}
/**
 * 引用传递，借用
 * */ 
fn addr(s: &String) -> usize {
    s.len()
}
/**
 * 可变引用
 */
fn modify_borrowing(s: &mut String) -> &String {
    s.push_str("ssssssss");
    return s;
}