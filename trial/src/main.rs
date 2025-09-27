fn main() {
    let mut x = 1;
    println!("{}", x);
    let y = x = 2;
    println!("{:?}, {:?}", x, y);
}
