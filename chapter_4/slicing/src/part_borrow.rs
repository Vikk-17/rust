fn main(){
    let s = String::from("Hello World");
    // cretes slices using [start_index..end_index]
    // let hello = &s[0..5];
    let len = s.len();
    let hello = &s[..5];
    // let world = &s[6..11];
    let world = &s[6..len];
}

fn first_word(s: &string) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
