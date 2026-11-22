mod checksum;
use checksum::Checksum;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file_path = Checksum::parser()?;
    let hash = Checksum::file_checksum(&file_path)?;
    println!("{:?}", hash);
    Ok(())
}
