fn main() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which is also
                                       // moves its reutrn value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // heppens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String { // gives_ownership will move its reutrn
                                 // value into the function that calls it 
    let some_string = String::from("yours");
    some_string // some string is returned and moves out of the calling function
}

// takes a String and returns one 
fn takes_and_gives_back(a_string: String) -> String {
    a_string // returned and moves out to the calling function
}
