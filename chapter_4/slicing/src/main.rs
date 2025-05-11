fn main() {
    let mut s = String::from("Rust Boy");
    let len = first_word(&s); 
    println!("{len}");
    s.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
