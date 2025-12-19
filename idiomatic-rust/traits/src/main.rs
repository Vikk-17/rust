// fn describe_type<T: SelfDescribing>(t: &T) -> String {
//     t.describe()
// }

// fn describe_type<T>() -> String
// where
//     T: SelfDescribing // <- Trait bound
// {
//     T::describe()
// }
//
// trait SelfDescribing {
//     fn describe() -> String;
// }
//
// struct Dog;
// impl SelfDescribing for Dog {
//     fn describe() -> String {
//         "happy little dog".into()
//     }
// }
//
// struct Cat;
// impl SelfDescribing for Cat {
//     fn describe() -> String {
//         "curious cat".into()
//     }
// }
// fn main() {
//     println!("I am a {}", describe_type::<Dog>());
//     println!("I am a {}", describe_type::<Cat>());
// }

fn describe_type<T>(t: &T) -> String
where
    T: SelfDescribing // <- Trait bound
{
    t.describe()
}

trait SelfDescribing {
    fn describe(&self) -> String;
}

struct Dog;
impl SelfDescribing for Dog {
    fn describe(&self) -> String {
        "happy little dog".into()
    }
}

struct Cat;
impl SelfDescribing for Cat {
    fn describe(&self) -> String {
        "curious cat".into()
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;

    println!("I am a {}", describe_type(&dog));
    println!("I am a {}", describe_type(&cat));
}
