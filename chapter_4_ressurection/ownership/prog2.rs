/**
 * If we want to deeply copy the heap data of the String
 * not just the stack data, we can use clone() function.
 */

fn main(){
    let mut s1 = String::from("Hello");
    let mut s = String::from("ahoy!");
    let s2 = s1.clone();
    // println!("{s}, world!");
    println!("s1 = {s1}, s2 = {s2}");
}
