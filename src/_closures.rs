use std::{ thread, time::Duration };

pub fn simulated_expensive_calculation(intensive: u32) -> u32 {
  println!("calculating slowly...");

  thread::sleep(Duration::from_secs(2));
  intensive
}
pub fn generate_workout(intensity: u32, random_number: u32) {
    // 闭包
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut expensive_result = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly only once...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
// 定义Cacher结构体，其中T为闭包，结构体中calculation存放闭包，value存放Option
struct Cacher <T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>
}
/**
 * Cacher::new 函数获取一个泛型参数 T，它定义于 impl 块上下文中并与 Cacher 结构体有着相同的 trait bound。
 * Cacher::new 返回一个在 calculation 字段中存放了指定闭包和在 value 字段中存放了 None 值的 Cacher 实例，因为我们还未执行闭包。
 * 当调用代码需要闭包的执行结果时，不同于直接调用闭包，它会调用 value 方法。这个方法会检查 self.value 是否已经有了一个 Some 的结果值；
 *  如果有，它返回 Some 中的值并不会再次执行闭包。
 *  如果 self.value 是 None，则会调用 self.calculation 中储存的闭包，将结果保存到 self.value 以便将来使用，并同时返回结果值。
 */
impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    } 
}