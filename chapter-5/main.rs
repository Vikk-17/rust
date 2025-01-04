/**
 * Defining and Instantiating Structs
 */

struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Using tuple Structs without named fields to create diff types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs without any fields
// These are called unit-like structs because they behave similarly to ()
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    // Returns a struct User
    User {
        active: true,
        // Because the parmas name and the struct field names are exactly the same
        // we can use the field init shorthand syntax to rewrite build_user so it 
        // behaves exactly the same but does not have the repetition of username
        // and email

        // username: username,
        username,
        //email: email,
        email,
        sign_in_count: 1,
    }
}

fn main(){
    let user1 = User {
        active: true,
        username: String::from("ghostExodus"),
        email: String::from("gE@gmail.com"),
        sign_in_count: 1,
    };
    // Note: the entire instance must be mutable, if we want to use mutable
    // Rust does not allow us to mark only certain fields as mutable.
    let user2 = build_user(
        String::from("abcd@gmail.com"),
        String::from("abcd"),
    );

    
    // struct update syntax, we can achieve the same effect with less code,
    // The syntax .. specifies that the remaining fields not explicitely set should
    // have the same value as the fields in the given instance.
    let user3 = User{
        email: String::from("another@gmail.com"),
        ..user1 // This must come last 
    };

    let black = Point(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}


/**
 * Ownership of Struct Data
 * For now I'm just leaving it, will cover it later.
 */
