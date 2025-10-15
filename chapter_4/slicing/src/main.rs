// fn main() {
//     let mut s = String::from("Rust Boy");
//     let len = first_word(&s); 
//     println!("{len}");
//     s.clear();
// }
//
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }




fn main() {
   let s = String::from("Rusty Boy");
   let len = part_of_words(&s);
   println!("{len:?}");
}

fn part_of_words(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == "0x32" {
            return i;
        }
    }
    s.len()
}
