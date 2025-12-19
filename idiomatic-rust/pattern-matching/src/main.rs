// fn what_type_of_integer_is_this<T: std::fmt::Display>(value: i32) {
//     match value {
//         1 => println!("This is number one"),
//         2 | 3 => println!("This is either two or three"),
//         4..=10 => println!("This is in between 4 to 10 (inclusive)"),
//         _ => println!("Some other kind of number"),
//     }
// }
//
// fn destructure_tuple(tuple: &(i32, i32, i32)) {
//     match tuple {
//         (first, ..) => println!("The first tuple element is: {first}")
//     }
//
//     match tuple {
//         (.., last) => println!("The last tuple element is: {last}")
//     }
//     match tuple {
//         (_, middle, _) => println!("The middle tuple element is: {middle}")
//     }
//     match tuple {
//         (first, middle, last) => {
//             println!("The whole tuple is: ({first}, {middle}, {last})")
//         }
//     }
// }

enum CatColor {
    Black,
    Red,
    Chocolate,
    Cinnamon,
    Blue,
    Cream,
    Cheshire,
}

struct Cat {
    name: String,
    color: CatColor,
}

fn match_on_black_cats(cat: &Cat) {
    match cat {
        Cat { name, color: CatColor::Black } => println!("This is a black cat named {name}"),
        Cat {name, color: _} => println!("{name} is not a black cat"),
    }
}

fn main() {
    // let x: i32 = 4;
    // what_type_of_integer_is_this::<i32>(x);
    let black_cat = Cat {
        name: String::from("Henry"),
        color: CatColor::Black,
    };

    let cheshire_cat = Cat {
        name: String::from("Penelope"),
        color: CatColor::Cheshire,
    };

    match_on_black_cats(&black_cat);
    match_on_black_cats(&cheshire_cat);
}
