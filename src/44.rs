// This Rust code is designed to be run in a Rust environment.
// It demonstrates basic operations using arithmetic operators,
// which are essential when working with numbers and literals.

fn main() {
    // Addition
    println!("{}", 5 + 3);

    // Subtraction
    let result = 7 - 2;
    println!("{}", result);

    // Multiplication
    let product = 4 * 6;
    println!("{}", product);

    // Division
    match (2.0 / 4.0, 8.0 / 2.0) {
        (x, y) => println!("Division: {}", x / y),
    }

    // Example of using a loop to print numbers from 1 to 10
    for i in 1..=10 {
        println!("{}", i);
    }
}
