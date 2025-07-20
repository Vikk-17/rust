// accept command line args
use std::{env, fs};

fn main() {
    // first value in the vecotr is binary then all the other passed args. 
    let args: Vec<String> = env::args().collect();

    let q1 = &args[1];
    let file_path = &args[2];

    println!("Searching for {q1}");
    println!("In file {file_path}");
   
    // takes the file path, open that file, and returns a value of type std::io::Result<String>
    // that contains file's contents

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
