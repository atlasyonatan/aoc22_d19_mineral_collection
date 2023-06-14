pub mod blueprint;
pub mod material;
pub mod simulation;
use simulation::State;

use crate::{blueprint::MaterialBlueprint, material::Material, simulation::simulate};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    process::ExitCode,
};

fn main() -> ExitCode {
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
        let result = simulate(&blueprint, 24, find_best_instruction);
        if let Err(error) = result {
            eprintln!("{}", error);
            return ExitCode::FAILURE;
        }
        let state = result.unwrap();
        println!("{:?}", state);
    }
    ExitCode::SUCCESS
}

fn find_best_instruction(
    blueprint: &MaterialBlueprint<Material, usize>,
    state: &State<Material, usize>,
) -> Option<Material> {
    Some(Material::Geode)
}
