/*
fn main(){
  let s = "hello";
  println!("{}", s);
}

*/

fn main(){

    // :: => operator
    // Allows us to namespace the particular ```from``` function
    // under the String type rather than using some sort of name 
    // like string_from.
    {
        let mut s =  String::from("hello");
        s.push_str(", world!");
        println!("{s}");
    } // This scope in now over, and s is no longer valid
      // Basically at this point rust already called a special function
      // called drop to return the allocated heap
    // println!("{s}"); // let's try
}
