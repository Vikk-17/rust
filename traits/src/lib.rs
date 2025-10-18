use std::{
    fmt::Display,
    cmp::PartialOrd,
};

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> 
where 
    T: Display + PartialOrd,
{
    fn new(x: T, y: T) -> Self {
        // using Self or the explicitely name of the function
        // Self{x, y}
        Pair {x, y} 
    }
    fn comp_display(&self) {
        if self.x > self.y {
            println!("The largest number is x: {}", self.x);
        } else {
            println!("The largest number is y: {}", self.y);
        }
    }

}

// impl<T: Display + PartialOrd> Pair<T> {
//     fn comp_display(&self) {
//         if self.x > self.y {
//             println!("The largest number is x: {}", self.x);
//         } else {
//             println!("The largest number is y: {}", self.y);
//         }
//     }
// }

// using trait bound
// impl<T> Pair<T>
// where 
//     T: Display + PartialOrd,
// {
//     fn comp_display(&self) {
//         if self.x > self.y {
//             println!("The largest number is x: {}", self.x);
//         } else {
//             println!("The largest number is y: {}", self.y);
//         }
//     }
// }
//
