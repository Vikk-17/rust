struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 1,
    }
}



fn main() {
    let user1 = build_user(
        String::from("Rust"),
        String::from("rust@gmail.com"),
    );
    println!("{0}", user1.active);
}
