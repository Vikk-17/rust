fn main(){
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String{

    let some_string = String::from("Ours");
    some_string // returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String{ // a_string comes into scope
    a_string    // a_string is returned and moves out to the calling function
}
