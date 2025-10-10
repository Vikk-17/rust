use std::ops::Add;

// pub fn add<T: Add<Output = T>>(a: T, b: T) -> T {
//     return a + b;
// }

pub fn add<T>(a: T, b: T) -> T
where 
    T: Add<Output = T>,
{
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_integers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_floats() {
        assert_eq!(add(2.1, 3.2), 5.3);
    }
}

fn main() {
    println!("Hello, world!");
}
