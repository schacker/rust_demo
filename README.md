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

#### 解引用
解引用使用 `*`运算符，先调用deref方法再接着使用`*`解引用的操作，只发生一次

#### 函数和方法的隐式解引用强制多态
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

#### Drop trait
在值离开作用域时执行特定代码的机制

#### 引用循环与内存泄漏是安全的，避免引用循环，可将Rc<T>变为Weak<T>

1. 调用 Rc::clone 会增加 Rc<T> 实例的 strong_count，和只在其 strong_count 为 0 时才会被清理的 Rc<T> 实例。
2. 也可以通过调用 Rc::downgrade 并传递 Rc<T> 实例的引用来创建其值的 弱引用（weak reference）。调用 Rc::downgrade 时会得到 Weak<T> 类型的智能指针。不同于将 Rc<T> 实例的 strong_count 加1，调用 Rc::downgrade 会将 weak_count 加1。Rc<T> 类型使用 weak_count 来记录其存在多少个 Weak<T> 引用，类似于 strong_count。
3. 两者区别在于 weak_count 无需计数为 0 就能使 Rc<T> 实例被清理。

强引用代表如何共享 Rc<T> 实例的所有权，但弱引用并不属于所有权关系。他们不会造成引用循环，因为任何弱引用的循环会在其相关的强引用计数为 0 时被打断。

>因为 Weak<T> 引用的值可能已经被丢弃了，为了使用 Weak<T> 所指向的值，我们必须确保其值仍然有效。为此可以调用 Weak<T> 实例的 upgrade 方法，这会返回 Option<Rc<T>>。如果 Rc<T> 值还未被丢弃，则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None。因为 upgrade 返回一个 Option<T>，我们确信 Rust 会处理 Some 和 None 的情况，所以它不会返回非法指针。

#### RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T> 的相似性
因为`Arc::new(Mutex::new(0))`是不可变的，不过可以获取内部值的可变引用，也就意味着`Mutex<T>`提供了内部可变性，就像Cell系列。使用 RefCell<T> 可以改变 Rc<T> 中的内容那样，同样的可以使用 Mutex<T> 来改变 Arc<T> 中的内容。