use std::{
    env,
    fs,
    process,
    error::Error,
};
use text_analyzer::{
    Config,
    FileStats
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if config.cmd == "stat" {
        // print the all metadata of the file
        let metadata = fs::metadata(&config.file_name)?;
        let file_type = FileStats::file_type(&metadata);
        let permissions = FileStats::get_permissions(&metadata).clone();
        let name = config.file_name; 
        let size_bytes = metadata.len();

        let file_stats: FileStats = FileStats { 
            name,
            file_type,
            size_bytes, 
            permissions 
        };
        println!("{}", serde_json::to_string_pretty(&file_stats)?);
    } else {
        println!("Expected to print other things");
    }

    Ok(())
}

