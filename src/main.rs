use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub mod blueprint;
use blueprint::parse::Blueprint;


fn main() {
    let file_path = "../input.txt";
    let path = Path::new(file_path);
    let file = File::open(path).unwrap();
    let blueprints = io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse::<Blueprint>())
        .map(Result::unwrap);
        
    println!("{:?}", blueprints);

}


