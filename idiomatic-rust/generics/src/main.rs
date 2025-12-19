// struct Container<T> {
//     value: T,
// }
//
// impl<T> Container<T> {
//     fn new(value: T) -> Self {
//         Self { value }
//     }
// }
//
// // Recursive struct
// #[derive(Clone)]
// struct ListItems<T>
// where
//     T: Clone
// {
//     data: Box<T>,
//     next: Option<Box<ListItems<T>>>,
// }
//
// fn main() {
//     // let str_container: Container<Option<String>> = Container { value: None };
//     // println!("{:#?}", str_container.value);
//
//     let none_container = Container::<Option<String>>::new(None);
//     println!("{:#?}", none_container.value);
// }
//

use std::ops::{Add, Mul, Sub, Div};
fn operation<T>(a: T, b: T, opr: &str) -> T
where 
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + std::cmp::PartialEq<i32>,
{
    // match opr {
    //     "+" => a + b,
    //     "-" => a - b,
    //     "*" => a * b,
    //     "/" if b!=0 => a / b,
    //     _ => todo!(),
    // }

    if opr == "+" {
        a + b
    } else if opr == "-" {
        a - b
    } else if opr == "*" {
        a * b
    } else if opr == "/" && b != 0 {
        a / b
    } else {
        T
    }
}

fn main() {
    let x = operation(5, 0, "/");
    println!("Result: {}", x);
}
