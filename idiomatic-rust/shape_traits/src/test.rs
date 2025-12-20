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
    fn get_width(&self) -> &Self::Output;
    fn get_height(&self) -> &Self::Output;
    fn get_area(&self) -> Self::Output;
}
impl<T> Rectangular for Rectangle<T>
where 
    T: std::ops::Mul<Output = T> + Clone
{
    type Output = T;

    fn get_width(&self) -> &T {
        &self.width
    }
    fn get_height(&self) -> &T {
        &self.height
    }
    fn get_area(&self) -> T {
        self.width.clone() * self.height.clone()
    }
}
impl<T> Rectangular for Square<T> 
where 
    T: std::ops::Mul<Output = T> + Clone
{
    type Output = T;

    fn get_width(&self) -> &T {
        &self.length
    }
    fn get_height(&self) -> &T {
        &self.length
    }
    fn get_area(&self) -> T {
        self.length.clone() * self.length.clone()
    }
}

fn main() {
    let rect = Rectangle::new(2, 3);
}
