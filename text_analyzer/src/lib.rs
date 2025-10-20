use std::fs;

fn read_file(name: String) {
   match fs::metadata(name) {
        Ok(data) => println!("{data:?}"),
        Err(e) => eprintln!("Error: {e}"),
   } 
}
