pub mod blueprint;
pub mod material;
pub mod simulation;

use crate::{
    blueprint::MaterialBlueprint,
    material::Material,
    simulation::{State, StepLog},
};

use std::{
    collections::{HashMap, HashSet},
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
    let mut qualitySum = 0;
    for (index, blueprint) in blueprints.enumerate() {
        println!(
            "Blueprint #{}: {}",
            index + 1,
            serde_json::to_string(&blueprint).unwrap()
        );
        let mut state = State::new();
        state.robots.insert(Material::Ore, 1usize);

        let total_time = 24;
        let mut log = StepLog::new();
        for time_left in (0..total_time).rev() {
            state.step(
                &blueprint,
                |current_state, options| {
                    strategy_priority_craft(options, current_state, &blueprint, time_left)
                },
                &mut log,
            );
            println!(
                "minute {}:\n\testimations: {:?}\n\trobots made: {:?}",
                total_time - time_left,
                estimate_materials(&state, time_left),
                &state.robots
            )
        }
        println!("\nSimulation Results:");
        //serde_json::to_string_pretty(&state).unwrap()
        println!(
            "\trobots made: (total: {}) {:?}",
            &state.robots
                .iter()
                .map(|(_, amount)| *amount)
                .sum::<usize>(),
            &state.robots
        );
        println!(
            "\tmaterials left: (total: {}) {:?}",
            &state.materials
                .iter()
                .map(|(_, amount)| *amount)
                .sum::<usize>(),
            &state.materials
        );
        println!(
            "\tmaterials collected: (total: {}) {:?}",
            &log.materials_collected
                .iter()
                .map(|(_, amount)| *amount)
                .sum::<usize>(),
            &log.materials_collected
        );
        println!(
            "\tmaterials spent: (total: {}) {:?}",
            &log.materials_spent
                .iter()
                .map(|(_, amount)| *amount)
                .sum::<usize>(),
            &log.materials_spent
        );
        
       
        if let Some(quality) = state.materials.get(&Material::Geode) {
            qualitySum += quality;
        }

        println!();
        // break;
    }

    println!("------------------\nQuality sum = {}", qualitySum);
}

fn strategy_priority_craft<'a>(
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
        .filter_map(|material| options.get(&material).map(|&choice| choice))
        .next()
}

fn strategy_one_of_each<'a>(
    options: impl Iterator<Item = &'a Material>,
    state: &State<Material, usize>,
    blueprint: &MaterialBlueprint<Material, usize>,
    time_left: usize,
) -> Option<&'a Material> {
    let priority = vec![
        Material::Geode,
        Material::Obsidian,
        Material::Clay,
        Material::Ore,
    ];
    let mut choice = None;
    for material in priority {
        if *state.robots.get(&material).unwrap_or(&0) < 1 {
            choice = Some(material)
        }
    }
    return match choice {
        Some(material) => options.filter(|&option| material == *option).next(),
        None => None,
    };
}

// fn strategy_look_ahead<'a>(
//     options: impl Iterator<Item = &'a Material>,
//     state: &State<Material, usize>,
//     blueprint: &MaterialBlueprint<Material, usize>,
//     time_left: usize,
// ) -> Option<&'a Material> {
//     options.map(Some).chain(None)
//     options.map(|option| (option, {
//         let mut state = state.clone();
//         state.step(blueprint, )

//     }));
//     let estimations = estimate(state, time_left);

//     None
// }

fn estimate_materials(
    state: &State<Material, usize>,
    time_left: usize,
) -> HashMap<Material, usize> {
    state
        .robots
        .iter()
        .map(|(material, collectors)| {
            (
                *material,
                collectors * time_left
                    + match state.materials.get(&material) {
                        Some(amount) => *amount,
                        None => 0,
                    },
            )
        })
        .collect()
}

fn estimate_robots(
    state: &State<Material, usize>,
    budget: &HashMap<Material, usize>,
    time_left: usize,
) -> HashMap<Material, usize> {
    state
        .robots
        .iter()
        .map(|(material, collectors)| {
            (
                *material,
                collectors * time_left
                    + match state.materials.get(&material) {
                        Some(amount) => *amount,
                        None => 0,
                    },
            )
        })
        .collect()
}

fn estimate_robot(
    recipe: &Vec<(usize, Material)>,
    budget: &HashMap<Material, usize>,
) -> Option<usize> {
    Some(
        recipe
            .iter()
            .map(|(material_cost, material)| match budget.get(material) {
                Some(&material_budget) if material_budget > 0 => material_budget / material_cost,
                _ => 0usize,
            })
            .min()?,
    )
}
