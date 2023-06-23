pub mod blueprint;
pub mod material;
pub mod simulation;

use crate::{blueprint::MaterialBlueprint, material::Material, simulation::State};

use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> () {
    let file_path = "../input.txt";
    let path = Path::new(file_path);
    let file = File::open(path).unwrap();
    let blueprints = io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse::<MaterialBlueprint<Material, usize>>())
        .map(Result::unwrap);
    for (index, blueprint) in blueprints.enumerate() {
        println!(
            "Blueprint #{}: {}",
            index + 1,
            serde_json::to_string(&blueprint).unwrap()
        );
        let mut state = State {
            active_robots: HashMap::from_iter(vec![(Material::Ore, 1)]),
            materials_collected: HashMap::new(),
        };
        for _ in 0..24 {
            state.step(&blueprint, |options| options.last());
        }
        println!("{:?}", &state)
    }
}

// fn find_best_instruction(
//     state: &State<Material, usize>,
// ) -> Option<Material> {

// }
