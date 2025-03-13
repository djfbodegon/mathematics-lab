
use std::{cmp, collections};

fn main() {
    let mut numbers = Vec::new();

    // fill the vector with random numbers
    for i in 1..10 {
        numbers.push(i);
    }

    // sort the vector in ascending order
    numbers.sort_by(|a, b| a.cmp(b));

    println!("{:?}", numbers);
}