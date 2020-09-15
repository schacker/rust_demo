## 小记

### String &String str &str 关系

* String 可变、堆上、字节缓冲区，可增长的、可变的、有所有权的、UTF-8 编码的字符串类型，内部实现为 `Vec<u8>` 的封装
* str 定长、不可变，解引用为堆引用，字面量为静态内存，真正写进rust核心语言的字符串
* &str 字符串切片（借用），不可变，也不放弃所有权？
* &String String的借用，指针，可传递，但不放弃所有权，等价于 &str
* & 为引用符，获取值但不获取所有权
* &mut 可变引用

* String::from 动态、堆上

* 想要一个字符串的只读视图，或者&str作为一个函数的参数，那就首选&str。如果想拥有所有权，想修改字符串那就用String
* 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
* 引用必须总是有效的。

### trait 类似其他语言interface

