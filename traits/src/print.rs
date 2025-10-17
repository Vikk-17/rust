// Have to use trait keyword to define a trait
trait Printable {
    // this trait includes the following functions
    fn print(&self);
}

// To implement a trait we need to use `impl` keyword along with `for`
// and we are implementing a trait for i32
impl Printabel for i32 {
    fn print(&self) {
        println!("{}", self);
    }
}


// Inheriting the Printable trait
impl PrintWithLabel: Printabel {
    fn print_with_label(&self, &str) {
        print!("{}", label);
        self.print();
    }
}

// Traits as a parameter
fn print_twice<T: PrintWithLabel>(value: T) {
    value.print_with_label("First");
    value.print_with_label("Second");
}
