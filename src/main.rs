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
        .map(|s| s.parse::<Blueprint<Material, usize>>())
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    println!("{:?}", blueprints);
}

extern crate strum;
#[macro_use]
extern crate strum_macros;
#[derive(EnumString, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
