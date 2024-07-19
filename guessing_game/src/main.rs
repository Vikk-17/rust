// We need to bring the io input/output library into the scope
// The io library comes from the standard library, known as std;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_num}");

    // The loop keyword creates an infinte loop
    loop {
        println!("Please enter your guess: ");

        // Store the variable
        // we use let to create the variable
        // Rust variables are immutable by default, once we give a value
        // the value won't change.
        // let apple = 5;
        // by using mut, the var will be mutable;
        // String::new() returns a new instance of String;
        // The :: syntax in the ::new line indicates that new
        // is an associated function of the String type.
        // The new function creates a new, empty string.
        // In the whole, the below line has created a mutable variable that
        // is currently bount to new, empty instance of String.
        let mut guess = String::new();

        // Receive user input:
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read lines");
        // Make it integer
        // Rust allows us to shadow the previous value of guess with a new one.
        // Shadowing lets us reuse the guess variable name rather than forcing us
        // to create two unique variables.
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        // read_lines puts whatever the user enters into the string we pass to it.
        // but it's also returns a result value. Result is an enumaration, often called
        // as enum, which is a type that can be in one of possible states.
        // We call each possible state a variant
        // Result's variants are Ok and Err. Err contains information about how or why
        // the operation failed.
        // An instance of Result has an expect method. If this instance of Result
        // is an Err value, expect will cause the program to crash and display the
        // message that we passed as an args to expect.
        //
        // Printing Values with println! Placeholders
        println!("You guessed: {}", guess);

        // When printing the value of a variable, the variable name can go
        // inside the curly brackets. When printing the result of evaluating
        // an expression, place empty curly brackets in thr format string, then
        // follow the format string with a comma-separated list of expressions
        // to print in each empty curly bracket Placeholder in the same order.
        /*
         * let x = 5;
         * let y = 10;
         * println!("x = {x} and y +2 = {}", y + 2); => x =5 and y + 2 = 12
         * */

        // Crate: binary crate and library crate
        // The one we r building is binary crate, which is an executable.
        // The rand crate is a library crate, which contains code that
        // is intended to be used in other programs and can't be executed
        // on it's own.
        //

        // Comparing
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
