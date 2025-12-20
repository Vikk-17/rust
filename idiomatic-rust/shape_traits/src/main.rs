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
struct Circle {
    radius: f64,
}
impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }
}
trait Measurable {
    type Output;
    fn dimension(&self) -> (&Self::Output, &Self::Output);
    fn area(&self) -> Self::Output;
}

impl<T> Measurable for Rectangle<T> 
where 
    T: Mul<Output = T> + Clone
{
    type Output = T;
    fn dimension(&self) -> (&T, &T) {
        (&self.width, &self.height)
    }
    fn area(&self) -> T {
        self.width.clone() * self.height.clone()
    }
}

impl Measurable for Circle 
{
    type Output = f64;
    fn dimension(&self) -> (&f64, &f64) {
        (&self.radius, &self.radius)
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let rect = Rectangle::new(2, 3);
    let (w, h) = rect.dimension();
    println!("Rectangle: width = {}, height = {}, area = {}", w, h, rect.area());

    let circle = Circle::new(3.0);
    let r = circle.dimension();
    println!("Circle: radius = {:#?}, area = {:#?}", r, circle.area());
}
