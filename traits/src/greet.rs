trait Greet {
    fn greet(&self);
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) {
        println!("Let's get rusty: {}", self.name);
    }
}

struct Student {
    person: Person, // inheriting the Person struct
    age: u8,
    address: String,
}

impl Greet for Student {
    fn greet(&self) {
        println!("I am {} and I am {} years old. Address: {}", self.person.name, self.age, self.address);
    }
}

fn main() {
    let p = Person {name: "Pirate".into()};
    p.greet();

    let s = Student {person: Person{name: "Alex".into()}, age: 25, address: "SF".into()};
    s.greet();
}
