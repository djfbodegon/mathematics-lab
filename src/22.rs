fn main() {
    let mut nums: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];

    for i in &mut nums {
        *i += 1.5;
    }

    println!("{:?}", nums);
}
