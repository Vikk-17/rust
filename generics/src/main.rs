// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0]; // fetch the first element and returning &i32
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     return largest;
// }
//
//
// fn largest_char(list: &[char]) -> &char {
//     // main reason to use &list[0] and not the list[0] is because it is 
//     // depend on your return type
//     let mut largest = &list[0]; // fetch the first element and return &char
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     return largest;
// }
//
// // Write the code for generics
// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         } 
//     }
//     return largest;
// } 


mod distance;
use distance::*;

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    //
    // let result = largest(&number_list);
    // println!("The largest number is {result}");
    //
    // let char_list = vec!['y', 'm', 'a', 'q'];
    //
    // let result = largest(&char_list);
    // println!("The largest char is {result}");
    //
    // // let decimal_list = vec![34.2, 50.4, 25.1, 100.5, 65.6];
    // let decimal_list: Vec<f64> = vec![34.1, 34.1, 34.1, 34.1, 34.1];
    //
    // let result = largest(&decimal_list);
    // println!("The largest number is {result}");

    let p1 = Point{x:3, y: 2};
    println!("x: {} and y: {}", p1.x, p1.y);
    let p2 = Point{x: 2, y: 4};
    println!("x: {} and y: {}", p1.x, p1.y);
    println!("Result: {}", p1.calcualtion(&p2));
}
