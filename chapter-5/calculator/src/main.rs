/**
 * We will start by using single variable to calculate the area of a rectangle.
 * Then refactor the program until we are using struct instead.
 */ 

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

// impl => implementation 
impl Rectangle {
    // self: &self or &self
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

/**
fn main() {
    
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    // println!(
    //    "The area of the reactangle is {} square pixels.",
    //    area1(&rect1)
    //);

    println!("rect1 is {rect1:#?}");
}

/*
fn area(width: u32, height: u32)-> u32 {
    width * height
}
*/ 

fn area1(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/ 
fn main(){
    /*
    let scale = 2;
    let rect1 = Rectangle{
        width: dbg!(30 * scale),
        height: 50,
    };
    // dbg! macro takes the ownership as opposed to println! macro that takes
    // references.
    dbg!(&rect1);
    // this macro can be really helpfun when we are trying to figure out 
    // what our code is doing.
    */ 
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels", rect1.area());
}
