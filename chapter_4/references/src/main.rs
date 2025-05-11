fn main() {
    let mut s1 = String::from("Rust");
    // let len = calculate_length(&s1);
    new_string(&mut s1);
    println!("New string is {s1}");
    // println!("The length of '{s1}' {len}");
}

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
fn new_string(s: &mut String){
    s.push_str(" Boy");
}
