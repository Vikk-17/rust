use std::{
    env,
    fs,
    process,
    error::Error,
};

use serde::Serialize;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if config.cmd == "stat" {
        // print the all metadata of the file
        let metadata = fs::metadata(&config.file_name)?;
        
        let file_type = if metadata.is_dir() {
            "directotry"
        } else if metadata.is_file() {
            "file"
        } else if metadata.is_symlink() {
            "symlink"
        } else {
            "unknown"
        };

        #[cfg(unix)]
        let permissions = {
            use::std::os::unix::fs::PermissionsExt;
            format!("{:o}", metadata.permissions().mode() & 0o777)
        };

        #[cfg(not(unix))]
        let permissions = if metadata.mode().readonly() {
            "read-only".to_string()
        } else {
            "read-write".to_string()
        };

        let file_stats: FileStats = FileStats { 
            name: config.file_name,
            file_type: file_type.to_string(),
            size_bytes: metadata.len(), 
            permissions: permissions.clone()
        };
        println!("{}", serde_json::to_string_pretty(&file_stats)?);
    } else {
        println!("Expected to print other things");
    }

    Ok(())
}

#[derive(Serialize)]
struct FileStats {
    name: String,
    file_type: String,
    size_bytes: u64,
    permissions: String,
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
