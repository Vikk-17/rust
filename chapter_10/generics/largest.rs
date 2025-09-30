fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}


// struct Point<T> {
//     x: T,
//     y: T,
// }

// By using this we can construct anything
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// let a = Point {x: 5, y: 10};
// let b = Point {x: 1.0, y: 4.0};
// let c = Point {x: 5, y: 4.0};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is: {}", result);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let result = largest(&number_list);
    // println!("The largest number is: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");
}
