/**
 * References and Borrowing
 * Rules:
 * -    At any given time, we can have either one mutable ref or any number of immutable ref.
 * -    References must always be valid.
 */

fn main(){
    let s1 = String::from("hello");
    let len = calculate_len(&s1);

    println!("The length of '{s1}' is {len}");

    let mut s2 = String::from("A Hacker");
    change(&mut s2);

    let r1 = &mut s2;
    // Error because we cannot borrow s2 as mutable ref more than once at a time.
    // let r2 = &mut s2;
    // let r3 = &s2; // Error
    // we also cannot have a mutable ref while we have an immutable one at the same value.

    println!("{s2}");

    // let r1 = &s2;
    // let r2 = &s2; // no Error
    // println!("{r1, r2}"); r1, r2 go out of scope 
    // let r3 = &mut s2; // no problem
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String){
    some_string.push_str(" Menifesto");
}
