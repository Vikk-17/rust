// fn main() {
//     let mut s1 = String::from("Rust");
//     // let len = calculate_length(&s1);
//     new_string(&mut s1);
//     println!("New string is {s1}");
//     // println!("The length of '{s1}' {len}");
// }
//
// // fn calculate_length(s: &String) -> usize {
// //     s.len()
// // }
// fn new_string(s: &mut String){
//     s.push_str(" Boy");
// }


fn main() {
    let mut s1 = String::from("Rust");
    s1.push_str(" is a good language");

    println!("s1: {s1:?}");

    s1.push_str(" check after print");
    println!("s1: {s1:?}");

    let len = send_the_string_to_another_function(&s1);
    println!("Length of the string: {len:?}");
    println!("s1: {s1:?}");
}

fn send_the_string_to_another_function(s: &String) -> usize {
    s.len()
}


