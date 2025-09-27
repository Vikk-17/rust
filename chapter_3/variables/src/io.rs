use std::io;

fn main() {

    let mut input = String::new();
    println!("Enter numbers separated by spaces: ");
    io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Sum = {}", nums[0] + nums[1]);
}
