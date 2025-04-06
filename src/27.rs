use std::collections::HashMap;

fn main() {
    let mut numbers = HashMap::new();
    numbers.insert(10, 2);
    numbers.insert(20, 3);
    println!("{:?}", numbers);
}
