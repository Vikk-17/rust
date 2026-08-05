// fn max_value<T>(a: T, b: T) -> T
// where 
//     T: std::cmp::PartialOrd
// {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }
//
// fn max_ref<'a, T>(a: &'a T, b: &'a T) -> &'a T
// where
//     T: std::cmp::PartialOrd
// {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }
//
// struct Pair<T> {
//     first: T,
//     second: T,
// }
//
// impl<T> Pair<T> {
//     fn new(first: T, second: T) -> Self {
//         Self {
//             first,
//             second,
//         }
//     }
//
//     fn first(&self) -> &T {
//         &self.first
//     }
//
//     fn second(&self) -> &T {
//         &self.second
//     }
//
//     fn swap(&mut self) {
//         std::mem::swap(&mut self.first, &mut self.second);
//     }
// }
//

struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T,U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Self{first, second}
    }
    fn first(&self) -> &T {
        &self.first
    }
    fn second(&self) -> &U {
        &self.second
    }
    fn replace_first(&mut self, value: T) {
        std::mem::replace(&mut self.first, value);
    }
    fn replace_second(&mut self, value: U) {
        std::mem::replace(&mut self.second, value);
    }
    fn map_first<F, R>(self, f: F) -> Pair<R, U>
    where 
        F: FnOnce(T) -> R
    {
        Pair {
            first: f(self.first),
            second: self.second
        }
    }
}

fn main() {
    // println!("{}", max_ref(&10, &20));
    // println!("{}", max_ref(&3.14, &2.71));
    // println!("{}", max_ref(&'a', &'z'));
    //
    // let a = String::from("apple");
    // let b = String::from("banana");
    //
    // println!("{}", max_value(&a, &b));
    // let mut p = Pair::new(10, 20);
    //
    // p.swap();
    //
    // assert_eq!(*p.first(), 20);
    // assert_eq!(*p.second(), 10);
}
