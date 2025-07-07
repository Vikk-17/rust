// defining the traits
trait Summery {
    // fn summery(&self) -> String;
    // Default return
    fn summery(&self) -> String{
        String::from("Summery")
    }
}

struct User {
    name: String,
    age: u32,
}

// No implementation => will use the default trait implementation
// impl Summery for User{}

impl Summery for User {
    fn summery(&self) -> String {
        format!("The age of {} is {}", self.name, self.age)
    }
}

// Traits as a params
// fn notify(item: &impl Summery) {
//     println!("{}", item.summery()); 
// }

// traits bound
fn notify<T: Summery>(item: &T){
    println!("{}", item.summery());
}

fn main() {
    let user = User {
        name: String::from("Alex"),
        age: 23,
    };
    // println!("{}", user.summery());
    notify(&user);
}
