pub mod blueprint;
pub mod material;
pub mod simulation;

use itertools::Itertools;

use crate::{
    blueprint::MaterialBlueprint,
    material::Material,
    simulation::{State, StepLog},
};

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

    let priority = vec![
        Material::Ore,
        Material::Clay,
        Material::Obsidian,
        Material::Geode,
    ];
    let mut quality_sum = 0;
    for (index, blueprint) in blueprints.enumerate() {
        println!(
            "Blueprint #{}: {}",
            index + 1,
            serde_json::to_string(&blueprint).unwrap()
        );
        let mut state = State::new();
        state.robots.insert(Material::Ore, 1usize);

        let total_time = 24;

        let max_costs = blueprint
            .0
            .iter()
            .flat_map(|recipie| {
                recipie
                    .1
                    .iter()
                    .map(|(amount, material)| (material, amount))
            })
            .into_grouping_map()
            .max();

        let robot_amount_goals: HashMap<_, _> = max_costs
            .into_iter()
            .map(|(material, amount)| (material, ((*amount as f32).sqrt() as usize)))
            .collect();
        println!("desired robots: {:?}", &robot_amount_goals);

        let mut log = StepLog::new();
        for _time_left in (0..total_time).rev() {
            state.step(
                &blueprint,
                |current_state, options| {
                    let desired_craft = priority.iter().find_map(|robot_kind| {
                        match robot_amount_goals.get(robot_kind) {
                            Some(desired_amount) => match current_state.robots.get(robot_kind) {
                                Some(current_amount) => match current_amount.cmp(desired_amount) {
                                    std::cmp::Ordering::Less => Some(robot_kind),
                                    _ => None,
                                },
                                None => Some(robot_kind),
                            },
                            None => Some(robot_kind),
                        }
                    })?;

                    // println!(
                    //     "Minute {}\n\tdesired_craft: {:?}",
                    //     total_time - time_left,
                    //     desired_craft
                    // );

                    let option = options.filter(|&option| option.eq(desired_craft)).next()?;

                    // println!("\tCrafted");

                    Some(option)
                    //
                    // strategy_priority_craft(options, current_state, &blueprint, time_left)
                },
                &mut log,
            );
            // println!(
            //     "minute {}:\n\testimations: {:?}\n\trobots made: {:?}",
            //     total_time - time_left,
            //     estimate_materials(&state, time_left),
            //     &state.robots
            // );
        }
        // println!("{}", serde_json::to_string_pretty(&state).unwrap());
        println!("\nSimulation Results:");
        println!(
            "\trobots made: (total: {}) {:?}",
            &state
                .robots
                .iter()
                .map(|(_, amount)| *amount)
                .sum::<usize>(),
            &state.robots
        );
        println!(
            "\tmaterials left: (total: {}) {:?}",
            &state
                .materials
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
        let quality = state
            .materials
            .get(&Material::Geode)
            .map(|amount| *amount)
            .unwrap_or_default();
        quality_sum += quality;
        println!("Quality: {}", quality);

        println!();
        break;
    }

    println!("------------------\nQuality sum = {}", quality_sum);
}
/*
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
*/
