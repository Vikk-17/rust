use std::{env, fs::File};
use sha2::{Sha256, Digest};
use std::path::Path;

pub struct Checksum;

impl Checksum {
    pub fn read_args() -> Vec<String> {
        let args: Vec<String> = env::args().collect();
        args
    }

    pub fn file_checksum(file_path: Path) -> Result<String, Box<dyn std::error::Error>> {
        // let contents = fs::read_to_string(file_path)
        //     .expect("Could not read the file");
        // file path
        // open the file and return a handler
        let mut file = File::open(file_path)
            .expect("Could not read the file");

        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher)?;
        let hash_bytes = hasher.finalize();

        Ok(hex::encode(hash_bytes))
    }
}
