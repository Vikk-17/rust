fn main() {
    let name: String = String::from("Souvik Chakrborty");
    let ans = first_word(&name);
    println!("{}", name);
    println!("ans is {}", ans);
}

fn first_word(str: &String) -> String {
    let mut ans = String::from("");

    for i in str.chars() {
        if i == ' ' {
            break;
        }
        ans.push_str(&i.to_string());
    }
    ans
}
