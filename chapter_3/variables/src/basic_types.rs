fn main() {
    let sum: u8 = 5 + 25;
    println!("Sum {sum:?}");

    let diff: f32 = 5.2 + 25.0; 
    println!("Difference {diff:?}");

    let quotient:f32 = 56.7 / 32.2;
    println!("quotient: {quotient:?}");

    let truncated:f32 = -5.0 / 3.0; // Results in -1
    println!("Truncated: {truncated:?}");
}
