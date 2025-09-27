use serde::Serialize;
use serde_json;


#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let u = User {
        id: 1,
        name: "Chakraborty".into(),
    };

    let json = serde_json::to_string(&u).unwrap();
    println!("{}", json);
}
