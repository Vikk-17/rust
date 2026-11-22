use std::{fs::File, io::Read};
use sha2::{Sha256, Digest};

pub struct Checksum;
impl Checksum {
    pub fn parser() -> Result<String, &'static str> {
        let args: Vec<String> = std::env::args().collect();

        if args.len() < 2 {
            return Err("Not enough argument");
        }

        Ok(args[1].clone())
    }

    pub fn file_checksum(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];
        loop {
            let byte_reads = file.read(&mut buffer)?;
            if byte_reads == 0 {
                break;
            }
            hasher.update(&buffer[..byte_reads]);
        }
        let result = hasher.finalize();
        let hex_string = hex::encode(result);
        Ok(format!("0x{}", hex_string))
    }
}
