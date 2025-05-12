#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // often used for constructor that will return a new instance of the struct.
    fn square(size: u32) -> Self {
        Self{
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 20,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 60,
    };
    
    let sq = Rectangle::square(3);
    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3: {}", rect1.can_hold(&rect3));

    // println!("{0:#?}", rect.area());
}

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

