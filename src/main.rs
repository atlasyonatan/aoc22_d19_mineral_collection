pub mod blueprint;
pub mod material;

use blueprint::MaterialBlueprint;
use material::Material;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let file_path = "../input.txt";
    let path = Path::new(file_path);
    let file = File::open(path).unwrap();
    let blueprints = io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse::<MaterialBlueprint<Material, usize>>())
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    println!("{}", serde_json::to_string(&blueprints).unwrap());
}
