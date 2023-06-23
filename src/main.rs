pub mod blueprint;
pub mod material;
pub mod simulation;

use crate::{blueprint::MaterialBlueprint, material::Material, simulation::State};

use std::{
    collections::HashSet,
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
        let mut state = State::new();
        state.robots.insert(Material::Ore, 1usize);

        for time_left in (0..24).rev() {
            state.step(&blueprint, |current_state, options| {
                find_best_instruction(options, current_state, &blueprint, time_left)
            });
        }
        println!("{}", serde_json::to_string_pretty(&state).unwrap())
    }
}

fn find_best_instruction<'a>(
    options: impl Iterator<Item = &'a Material>,
    state: &State<Material, usize>,
    blueprint: &MaterialBlueprint<Material, usize>,
    time_left: usize,
) -> Option<&'a Material> {
    let options = options.collect::<HashSet<_>>();
    let priority = vec![
        Material::Geode,
        Material::Obsidian,
        Material::Clay,
        Material::Ore,
    ];
    priority
        .into_iter()
        .filter_map(|material| options.get(&material).map(|choise| *choise))
        .next()
}
