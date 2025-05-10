fn main(){
    let _s1 = gives_ownership();
    
    let s2 = String::from("Boy");
    let _s3 = take_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("Rust");
    some_string // return some_string
}

fn take_and_gives_back(some_string: String) -> String {
    some_string
}
