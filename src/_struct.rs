
// 泛型参数类型结构体
#[derive(Debug)] //调试信息注解
pub struct Anything<T> {
    x: T,
    y: T
}
#[derive(Debug)] //调试信息注解
pub struct ComplexAnything<T, U> {
    x: T,
    y: U
}
pub struct User {
    pub name: String,
    pub email: String,
    pub age: i8
}
pub struct Color(i8, i8, i8);
#[derive(Debug)] //调试信息注解
pub struct Rect {
    pub width: u32,
    pub height: u32,
}
// 方法语法
impl Rect {
    // 自身不可变借用
    pub fn area(&self) -> u32 {
        self.height * self.width
    }
    // 方法，类比对象原型方法，挂载结构体实例
    pub fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 函数，类比类的静态方法，挂载结构体命名空间
    pub fn square(size: u32) -> Rect {
        Rect { width: size, height: size }
    }
}

pub fn area(dimensions: &Rect) -> u32 {
    &dimensions.width * &dimensions.height // dimensions.width * dimensions.height 等价
}

pub fn structfn() {
    let user1 = User {
        name: String::from("schacker"),
        email: String::from("huangwei@ke.com"),
        age: 18
    };
    let user2 = User {
        name: String::from("schacker1"),
        ..user1
    };
    let mut user3 = User {
        email: String::from("sss@c"),
        ..user2
    };
    let color = Color(23,23,23);
    user3.age = 12;
    
    let anything = Anything {
        x: String::from("anythingone"),
        y: String::from("anythingtwo"),
    };
    let complex_anything = ComplexAnything {
        x: 1,
        y: 3.123
    };
    println!("{}, {}, {}, {}, {:?}, {:?}", user2.email, user3.age, user3.email, color.0, anything, complex_anything);
}
