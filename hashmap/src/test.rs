enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
    let mut output:Vec<String> = Vec::new();
    for (s, command) in input {
        match command {
            Command::Uppercase => output.push(s.to_uppercase()),
            Command::Trim => output.push(s.trim().to_string()),
            Command::Append(n) => output.push(format!("{}{}", s, s.repeat(n))),
        }
    }
    output
}


fn main() {
    let input = vec![
        ("hello".to_string(), Command::Uppercase),
        (" all roads lead to rome! ".to_string(), Command::Trim),
        ("foo".to_string(), Command::Append(1)),
        ("bar".to_string(), Command::Append(5)),
    ];
    let output = transformer(input);
    println!("{:?}", output);
}
