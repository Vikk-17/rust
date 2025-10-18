use std::ops::{Add, Mul};

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where 
    T:Add<Output = T> + Mul<Output = T> + Copy,
{
    pub fn calcualtion(&self, other: &Point<T>) -> T {
        return (self.x * other.x) + (self.y * other.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_int() {
        let p1 = Point{x: 3, y: 2};
        let p2 = Point{x: 1, y: 4};
        let result = p1.calcualtion(&p2);
        assert_eq!(result, 11);
    }

    #[test]
    fn calc_float() {
        let p1 = Point{x: 3.0, y: 2.0};
        let p2 = Point{x: 1.0, y: 4.0};
        let result = p1.calcualtion(&p2);
        assert_eq!(result, 11.0);
    }
}
