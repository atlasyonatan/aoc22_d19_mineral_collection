pub mod blueprint;
pub mod material;

use blueprint::MaterialBlueprint;
use material::Material;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use strum::IntoEnumIterator;

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

fn maximize_geodes(blueprint: &MaterialBlueprint<Material, usize>, mut time: usize) -> usize {
    let mut collected = Material::iter()
        .map(|material| (material, 0usize))
        .collect::<HashMap<_, _>>();
    let mut robots = Material::iter()
        .map(|material| (material, 0usize))
        .collect::<HashMap<_, _>>();

    let best_move = || -> Option<Material> {
        todo!()
        // if blueprint.0.get(&Material::Geode).unwrap(){

        // }
    };
    for minute in 0..time {
        //create robots
        match best_move() {
            Some(material) => {
                *robots.get_mut(&material).unwrap() += 1;
                for (amount, material) in blueprint.0.get(&material).unwrap() {
                    *collected.get_mut(material).unwrap() -= amount;
                }
            }
            None => (),
        }

        //robots collect
        for (&material, &amount) in robots.iter() {}
    }
    *collected.get(&Material::Geode).unwrap()
}
