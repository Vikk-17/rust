fn main() {
    /*
    let mut x: u32 = 500;
    println!("The value of x is: {x}");
    x = 1000;
    println!("The value of x is: {x}");
    */ 
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
