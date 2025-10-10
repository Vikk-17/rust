use std::fs::File;

// we need this because of Result::Ok and Result::Err, otherwise we have to mention the whole thing
use std::io::prelude::*;
use std::io::ErrorKind;
// fn main() -> std::io::Result<()> {
//     let mut file = File::create("foo.txt")?;
//     file.write_all(b"Testing")?;
//
//     Ok(())
// }

// fn main() -> std::io::Result<()> {
//     let mut file = File::open("foo.txt")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//
//     println!("Contents: {contents:?}");
//     Ok(())
// }


fn read_username_from_file() -> Result<String, std::io::Error> {
    let file = File::open("user.txt");
    let mut fhandler = match file {
        Ok(fh) => fh,
        Err(error) => return Err(error),
    };

    let mut username = String::new();
    match fhandler.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }

}


fn read_username() -> Result<String, std::io::Error> {
    // let mut file = File::open("user.txt")?;
    let mut username = String::new();
    // file.read_to_string(&mut username)?;
    File::open("user.txt")?.read_to_string(&mut username)?;
    Ok(username)
}


fn main() {
    let file = File::open("main.txt");

    let greeting_file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("main.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem createing the file: {e:?}")
            }
            _ => panic!("Problem opening the file: {error:?}"),
        } 
    };
}
