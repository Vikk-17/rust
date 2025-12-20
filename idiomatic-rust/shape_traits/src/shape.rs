use std::ops::Mul;

struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T> Rectangle<T> {
    fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}

struct Square<T> {
    length: T,
}

impl<T> Square<T> {
    fn new(length: T) -> Self {
        Self { length }
    }
}

trait Rectangular {
    type Output;
    fn width(&self) -> &Self::Output;
    fn height(&self) -> &Self::Output;
    fn area(&self) -> Self::Output;
}

impl<T> Rectangular for Rectangle<T> 
where 
    T: Mul<Output = T> + Clone
{
    type Output = T;
    fn width(&self) -> &T {
        &self.width
    }
    fn height(&self) -> &T {
        &self.height
    }
    fn area(&self) -> T {
        self.width.clone() * self.height.clone()
    }
}

impl<T> Rectangular for Square<T> 
where 
    T: Mul<Output = T> + Clone
{
    type Output = T;
    fn width(&self) -> &T {
        &self.length
    }
    fn height(&self) -> &T {
        &self.length
    }
    fn area(&self) -> T {
        self.length.clone() * self.length.clone()
    }
}

fn main() {
    let rect = Rectangle::new(2, 3);
    let square = Square::new(2);
    println!("Rectangle: width: {}, height: {}, area: {}", rect.width(), rect.height(), rect.area());
    println!("Square: width: {}, height: {}, area: {}", square.width(), square.height(), square.area());
}

