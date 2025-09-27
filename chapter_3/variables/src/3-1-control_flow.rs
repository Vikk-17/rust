use std::io;

fn main() {

    let mut input = String::new();
    let mut number_input = String::new();

    println!("Enter your name: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("This is fun, ya {}", input.trim());

    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut number_input)
        .unwrap();

    let number: i32 = number_input.trim().parse().expect("Not valid");
    println!("I have entered: {}", number);
}
