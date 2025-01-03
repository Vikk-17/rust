fn main(){
    let s = String::from("hello"); // comes into the scope
    
    // s value moves into the function...
    takes_owenership(s);
    // so s is no longer valid here
    
    let x = 5; // comes into scope

    // move into the function
    makes_copy(5);
    // but i32 is Copy, so it's okay to still use x afterward.
}


fn takes_owenership(some_string: String) { // some_string comes into the scope
    println!("{some_string}");
} // some_string goes out of the scope and 'drop' is called, the backing memory
  // is freed


fn makes_copy(some_integer: i32){
    println!("{some_integer}");
} // goes out of scope, nothing special happens.
