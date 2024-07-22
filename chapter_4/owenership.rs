fn main(){
    let s = String::from("hello"); // s comes to scope
    // s's value moves to the function
    takes_ownership(s);
    // ... and so is no longer valid here
    //  println!("{s}");
    let x = 5; // comes to scope
    makes_copy(x); // x would copy into the function
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    println!("{x}");
} // here, x goes out of scope, then s.

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // here, some_string goes out of scope and 'drop' is called
  // The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // Nothing special happens.
