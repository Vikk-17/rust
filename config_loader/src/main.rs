mod model;
use model::read_json_file;


fn main() {
   let data = read_json_file("config.json").unwrap();
   println!("{:?}", data._id);
}
