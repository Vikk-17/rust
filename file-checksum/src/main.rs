mod checksum;
use checksum::Checksum;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Checksum::read_args();
    println!("{:#?}", args);
    let file_name = args[1].parse();

    // generate the hash
    let hash = Checksum::file_checksum(file_name)?;
    println!("{:#?}", hash);

    Ok(())
}
