use crate::test::subtest::Point;
pub mod test;


fn main() {
    let p1 = Point {
        x: 3,
        y: 2,
    };


    println!("The point is: ({}, {})", p1.x, p1.y);
}
