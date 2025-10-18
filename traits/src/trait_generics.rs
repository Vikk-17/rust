trait Greet {
    fn greet(&self) -> String;
}

struct Person {
    name: String,
}

struct Robot;

// Take traits as an parameter
// this is called trait bound
// basically it's the sytatic sugar 
// fn say_hello<T: Greet>(thing: &T) {
//    println!("{}", thing.greet()); 
// }

fn say_hello<T>(thing: &T)
where
    T: Greet,
{
   println!("{}", thing.greet()); 
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("This is : {}", self.name)
    }
}

impl Greet for Robot {
    fn greet(&self) -> String {
        "Testing this in the prod".into()
    }
}

fn main() {
    let person = Person {name: "Alex".to_string()};
    let robot = Robot;

    say_hello(&person);
    say_hello(&robot);
}
