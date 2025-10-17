trait Describe {
    fn describe(&self);
}

struct Dog;
struct Car;

impl Describe for Dog {
    fn describe(&self) {
        println!("Can bark");
    }
}

impl Describe for Car {
    fn describe(&self) {
        println!("Can drive");
    }
}

fn main() {
    let d = Dog;
    let c = Car;

    d.describe();
    c.describe();
}
