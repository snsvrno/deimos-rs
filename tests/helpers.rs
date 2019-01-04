
use std::fs::File;
use std::io::Read;

pub fn load_file(file_name : &str) -> String {
    let mut file = File::open(&format!("tests/lua/{}.lua",file_name)).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    contents
}