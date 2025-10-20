use std::{
    env,
    fs,
    process,
    error::Error,
};
// use text_analyzer::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if config.cmd == "stat" {
        // print the all metadata of the file
        let metadata = fs::metadata(config.file_name)?;
        println!("{metadata:?}");
    } else {
        println!("Expected to print other things");
    }

    Ok(())
}


// The command line expectation
struct Config {
    cmd: String,
    file_name: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough argument");
        }
        let cmd = args[1].clone();
        let file_name = args[2].clone();


        Ok(Config {
            cmd,
            file_name,
        })
    }
}
