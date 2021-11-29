## 小记

### 所有权

- Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
- 值在任一时刻有且只有一个所有者。
- 当所有者（变量）离开作用域，这个值将被丢弃。
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


### 闭包使用经验

- `FnOnce` 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
- `FnMut` 获取可变的借用值所以可以改变其环境
- `Fn` 从其环境获取不可变的借用值

- 当创建一个闭包时，Rust 根据其如何使用环境中变量来推断我们希望如何引用环境。由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 FnOnce 。那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 FnMut ，而不需要对被捕获的变量进行可变访问的闭包则也实现了 Fn

- 如果你希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 move 关键字。这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用。

```rust
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```
这里的x被移进了闭包，闭包获取了x的所有权，所以后续就不能再使用了

### 智能指针

> 考虑到智能指针是一个在 Rust 经常被使用的通用设计模式，本章并不会覆盖所有现存的智能指针。很多库都有自己的智能指针而你也可以编写属于你自己的智能指针。这里将会讲到的是来自标准库中最常用的一些：

  - Box<T>，用于在堆上分配值
  - Rc<T>，一个引用计数类型，其数据可以有多个所有者，只适用于单线程
  - Ref<T> 和 RefMut<T>，通过 RefCell<T> 访问。（ RefCell<T> 是一个在运行时而不是在编译时执行借用规则的类型）。
  
> 另外我们会涉及 内部可变性（interior mutability）模式，这是不可变类型暴露出改变其内部值的 API。我们也会讨论 引用循环（reference cycles）会如何泄漏内存，以及如何避免。

#### 内部可变性

> 内部可变性（Interior mutability）是 Rust 中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的。为了改变数据，该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则。我们还未讲到不安全代码；第十九章会学习它们。当可以确保代码在运行时会遵守借用规则，即使编译器不能保证的情况，可以选择使用那些运用内部可变性模式的类型。所涉及的 unsafe 代码将被封装进安全的 API 中，而外部类型仍然是不可变的。

> 因为一些分析是不可能的，如果 Rust 编译器不能通过所有权规则编译，它可能会拒绝一个正确的程序；从这种角度考虑它是保守的。如果 Rust 接受不正确的程序，那么用户也就不会相信 Rust 所做的保证了。然而，如果 Rust 拒绝正确的程序，虽然会给程序员带来不便，但不会带来灾难。RefCell<T> 正是用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候。

> 这里的`RefCell`有点，人比机器分析得更靠谱、更智能

如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：

Rc<T> 允许相同数据有`多个所有者`；Box<T> 和 RefCell<T> 有`单一所有者`。
Box<T> 允许在`编译时`执行`不可变或可变`借用检查；Rc<T>仅允许在`编译时`执行`不可变`借用检查；RefCell<T> 允许在`运行时`执行`不可变或可变`借用检查。
因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
在不可变值内部改变值就是 内部可变性 模式。

### 解引用
解引用使用 `*`运算符，先调用deref方法再接着使用`*`解引用的操作，只发生一次

### 函数和方法的隐式解引用强制多态
1. 只能工作在实现了Deref trait的类型上
2. 解引用强制多态将一种类型（A）隐式转换为另外一种类型（B）的引用，因为A类型实现了Deref trait，并且其关联类型是B类型。比如，解引用强制多态可以将&String转换为&str，因为类型String实现了Deref trait并且其关联类型是str
3. 解引用强制多态都发生在编译阶段，所以并没有运行时消耗

类似于使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。

Rust 在发现类型和 trait 实现满足三种情况时会进行解引用强制多态：

当 T: Deref<Target=U> 时从 &T 到 &U。
当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
当 T: Deref<Target=U> 时从 &mut T 到 &U。
头两个情况除了可变性之外是相同的：第一种情况表明如果有一个 &T，而 T 实现了返回 U 类型的 Deref，则可以直接得到 &U。第二种情况表明对于可变引用也有着相同的行为。

第三个情况有些微妙：Rust 也会将可变引用强转为不可变引用。但是反之是 不可能 的：不可变引用永远也不能强转为可变引用。因为根据借用规则，如果有一个可变引用，其必须是这些数据的唯一引用（否则程序将无法编译）。将一个可变引用转换为不可变引用永远也不会打破借用规则。将不可变引用转换为可变引用则需要数据只能有一个不可变引用，而借用规则无法保证这一点。因此，Rust 无法假设将不可变引用转换为可变引用是可能的。

### Drop trait
在值离开作用域时执行特定代码的机制

### 引用循环与内存泄漏是安全的，避免引用循环，可将Rc<T>变为Weak<T>

1. 调用 Rc::clone 会增加 Rc<T> 实例的 strong_count，和只在其 strong_count 为 0 时才会被清理的 Rc<T> 实例。
2. 也可以通过调用 Rc::downgrade 并传递 Rc<T> 实例的引用来创建其值的 弱引用（weak reference）。调用 Rc::downgrade 时会得到 Weak<T> 类型的智能指针。不同于将 Rc<T> 实例的 strong_count 加1，调用 Rc::downgrade 会将 weak_count 加1。Rc<T> 类型使用 weak_count 来记录其存在多少个 Weak<T> 引用，类似于 strong_count。
3. 两者区别在于 weak_count 无需计数为 0 就能使 Rc<T> 实例被清理。

强引用代表如何共享 Rc<T> 实例的所有权，但弱引用并不属于所有权关系。他们不会造成引用循环，因为任何弱引用的循环会在其相关的强引用计数为 0 时被打断。

>因为 Weak<T> 引用的值可能已经被丢弃了，为了使用 Weak<T> 所指向的值，我们必须确保其值仍然有效。为此可以调用 Weak<T> 实例的 upgrade 方法，这会返回 Option<Rc<T>>。如果 Rc<T> 值还未被丢弃，则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None。因为 upgrade 返回一个 Option<T>，我们确信 Rust 会处理 Some 和 None 的情况，所以它不会返回非法指针。

### RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T> 的相似性
因为`Arc::new(Mutex::new(0))`是不可变的，不过可以获取内部值的可变引用，也就意味着`Mutex<T>`提供了内部可变性，就像Cell系列。使用 RefCell<T> 可以改变 Rc<T> 中的内容那样，同样的可以使用 Mutex<T> 来改变 Arc<T> 中的内容。

### 模式和匹配
- match 分支
```rust
match VALUE {
  PATTERN => EXPRESSION
  PATTERN => EXPRESSION
  PATTERN => EXPRESSION
  _       -> EXPRESSION
}
```

- if let 表达式

> 一般用在只关心特定个别条件的情况

- while let 条件循环
```rust
let mut stack = vec![1,2,3];

while let Some(top) = stack.pop() {
  println!("{}", top);
}
```

- for 循环
```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() { //解构
    println!("{} is at index {}", value, index);
}
```

- let 模式？

```rust
let (x, y, z) = (1, 2, 3);
```

- 函数参数
```rust
// x 部分也是一个模式
fn foo(x: i32) {
    // 代码
}
// 解构
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

```

#### 模式语法

- 匹配字面量
- 匹配命名变量
```rust
  let x = Some(5);
  let y = 10;

  match x {
      Some(50) => println!("Got 50"),
      Some(y) => println!("Matched, y = {:?}", y),
      _ => println!("Default case, x = {:?}", x),
  }

  println!("at the end: x = {:?}, y = {:?}", x, y);
```
- 多个模式
```rust
let x = 1;

match x {
  1 | 2 => println!("one or two"),
  3 => println!("three"),
  _ => println!("anything"),
}
```
- 通过..= 匹配值的范围
```rust
let x = 5;

match x {
  1..=5 => println!("one through five"),
  _ => println!("something else"),
}

let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```
- 解构并分解值
```rs
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let p = Point { x: 0, y: 7 };

  let Point { x: a, y: b } = p;
  assert_eq!(0, a);
  assert_eq!(7, b);
}

struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let p = Point { x: 0, y: 7 };

  let Point { x, y } = p;
  assert_eq!(0, x);
  assert_eq!(7, y);
}

fn main() {
  let p = Point { x: 0, y: 7 };

  match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }
}
```
- 解构枚举
```rust
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

fn main() {
  let msg = Message::ChangeColor(0, 160, 255);

  match msg {
    Message::Quit => {
      println!("The Quit variant has no data to destructure.")
    }
    Message::Move { x, y } => {
      println!(
          "Move in the x direction {} and in the y direction {}",
          x,
          y
      );
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
      println!(
        "Change the color to red {}, green {}, and blue {}",
        r,
        g,
        b
      )
    }
  }
}
```
- 解构嵌套的结构体和枚举
```rust
enum Color {
  Rgb(i32, i32, i32),
  Hsv(i32, i32, i32),
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(Color),
}

fn main() {
  let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

  match msg {
    Message::ChangeColor(Color::Rgb(r, g, b)) => {
      println!(
        "Change the color to red {}, green {}, and blue {}",
        r,
        g,
        b
      )
    }
    Message::ChangeColor(Color::Hsv(h, s, v)) => {
      println!(
        "Change the color to hue {}, saturation {}, and value {}",
        h,
        s,
        v
      )
    }
    _ => ()
  }
}
```
- 解构结构体和元组
```rust
let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```
- 忽略模式中的值 _ 忽略整个值
```rust
fn foo(_: i32, y: i32) {
  println!("This code only uses the y parameter: {}", y);
}
```
- 嵌套的 _ 忽略部分值
```rust
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
  (Some(_), Some(_)) => {
    println!("Can't overwrite an existing customized value");
  }
  _ => {
    setting_value = new_setting_value;
  }
}

println!("setting is {:?}", setting_value);
```
- 用..忽略剩余值
```rust
struct Point {
  x: i32,
  y: i32,
  z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
  Point { x, .. } => println!("x is {}", x),
}
```
- 匹配守卫提供的额外条件
```rust
let num = Some(4);

match num {
  Some(x) if x < 5 => println!("less than five: {}", x),
  Some(x) => println!("{}", x),
  None => (),
}
```
- @ 绑定 
> 允许创建一个存放值的变量的同时，测试其值是否匹配模式
```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
```

### 高级特征-返回闭包
> 闭包表现为 trait，这意味着不能直接返回闭包。对于大部分需要返回 trait 的情况，可以使用实现了期望返回的 trait 的具体类型来替代函数的返回值。但是这不能用于闭包，因为他们没有一个可返回的具体类型；例如不允许使用函数指针 fn 作为返回值类型
```rust
/**
 * 编译失败
 */
fn returns_closure() -> Fn(i32) -> i32 {
  |x| x + 1
}
/*
 * 通过trait对象包装
 */
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

### 高级特征-宏
> 使用macro_rules!声明宏，和三种过程宏

- 自定义`#[derive]`宏在结构体和枚举上指定通过`derive`属性添加的代码
- 类属性(Attribute-like)宏定义可用于任一项的自定义属性
- 类函数宏看起来像函数不过作用于作为参数传递的token

```rust
#[macro_export] // 标明宏应该可用，且可被引用
macro_rules! vec { // 定义+名称
// ( $( $x:expr ),* ) 称为`单边模式`，复杂的会有多个单边模式，单边模式匹配的是代码结构而不是值

// 首先，一对括号包含了整个模式。接下来是美元符号（ $ ），后跟一对括号，捕获了符合括号内模式的值以用于替换后的代码。$() 内则是 $x:expr ，其匹配 Rust 的任意表达式，并将该表达式记作 $x。
// $() 之后的逗号说明一个可有可无的逗号分隔符可以出现在 $() 所匹配的代码之后。紧随逗号之后的 * 说明该模式匹配零个或更多个 * 之前的任何模式。
  ( $( $x:expr ),* ) => {
    {
      let mut temp_vec = Vec::new();
      $(
          temp_vec.push($x);
      )*
      temp_vec
    }
  };
}
```
#### 宏和函数的区别
- 本质上宏是为写其它代码而写代码的方式，即所谓的`元编程`
- 调用宏之前必须定义它或将其引入作用域，函数则可以在任何地方定义和调用

#### 从属性生成代码的过程宏
> 定义过程宏的函数接受一个 TokenStream 作为输入并产生一个 TokenStream 作为输出。
宏的核心：宏所处理的源代码组成了输入 TokenStream，同时宏生成的代码是输出 TokenStream。最后，函数上有一个属性；这个属性表明过程宏的类型。在同一 crate 中可以有多种的过程宏。

```rust
use proc_macro;

#[some_attribute] // 使用特定宏的占位符
pub fn some_name(input: TokenStream) -> TokenStream {
}
```