pub fn test_iterators() {
    let vec = vec![1,2,3,4,5,6,7,7];
    let iterator = vec.iter();
    let result: Vec<_> = iterator.map(|x| x+1).collect();
    println!("{:?}", result);
}
#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String
}
struct Counter {
    count: u32
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
pub fn test_filter_iterators() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("nick") },
        Shoe { size: 11, style: String::from("nick1") },
        Shoe { size: 12, style: String::from("nick2") },
        Shoe { size: 13, style: String::from("nick3") },
        Shoe { size: 12, style: String::from("nick4") },
        Shoe { size: 10, style: String::from("nick5") }
    ];
    let result = shoes_in_my_size(shoes, 10);
    println!("{:?}", result);
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}