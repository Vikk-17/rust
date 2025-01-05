/**
 * All functions defined within an impl block are called associated functions.
 * Because they are associated with the type named after the impl. 
 * We can define associated functions that don't have self as their 
 * first params (and thus are not methods) because they don't need an instance of the type to work
 * with.
 */



#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let sq = Rectangle::square(3);
    println!("The self is: {}", sq);
}
