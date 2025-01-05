struct Rectangle{
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
}

/**
 * When we give a method the same name as a field we want it to only return the 
 * value in the field and do nothing else. This are called getters. And Rust does
 * not implement them automatically for struct fields as some other lang does.
 * These are useful because we can make the field private but the method public
 * and thus enable read-only access to that field as a part of the type's public API.
 

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}
*/



fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };

    /*
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    */ 
    // These are the same
    // (&rect1).can_hold(&rect2); 
    // rect1.can_hold(&rect2);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
