use std::io;

fn main() -> std::io::Result<()>{
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {a:?}");

    let mut index = String::new();
    io::stdin().read_line(&mut index)?;

        // .expect("Falied to get the input");

    // println!("index: {index:?}");
    let index: u8 = index.trim().parse()
        .expect("Didn't get the number");
    let element = a[2];
    println!("index: {index:?} and element: {element:?}");
    Ok(())
}
