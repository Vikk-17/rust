// accept command line args
use std::{env, process};
use cmd::Config;

fn main() {
    // first value in the vector is binary then all the other passed args. 
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // takes the file path, open that file, and returns a value of type std::io::Result<String>
    // that contains file's contents

    if let Err(e) = cmd::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
