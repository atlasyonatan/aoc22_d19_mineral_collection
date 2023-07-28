pub mod blueprint;
pub mod mineral;
pub mod pack;

use crate::blueprint::Blueprint;
use crate::mineral::Mineral;
use crate::pack::Pack;

use mineral::MineralArray;
use strum::IntoEnumIterator;

use std::{
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
        .map(|s| s.parse::<Blueprint<usize>>())
        .map(Result::unwrap);

    // let priority = vec![
    //     Mineral::Ore,
    //     Mineral::Clay,
    //     Mineral::Obsidian,
    //     Mineral::Geode,
    // ];
    let mut quality_sum = 0;
    for (index, blueprint) in blueprints.enumerate() {
        let index = index + 1;
        // println!(
        //     "Blueprint #{}: {}",
        //     index,
        //     serde_json::to_string(&blueprint).unwrap()
        // );
        let mut pack = Pack::default();
        pack.robots[Mineral::Ore] = 1;

        let total_time = 24;
        let quality = max_geodes(
            &blueprint,
            pack,
            total_time,
            &MineralArray::default(),
            false,
        );
        println!("Quality: {}", quality);
        quality_sum += index * quality;
        break;
        // let total_costs = blueprint
        //     .0
        //     .iter()
        //     .flat_map(|recipie| {
        //         recipie
        //             .1
        //             .iter()
        //             .map(|(amount, material)| (material, amount))
        //     })
        //     .into_grouping_map()
        //     .fold(0, |accumulator, _, value| accumulator + value);
        // .into_iter()
        // .map(|(material, minmax)| (material, minmax.into_option().unwrap()));

        // let robot_amount_goals: HashMap<_, _> = total_costs
        //     .into_iter()
        //     .map(|(material, amount)| (material, (((amount as f32).sqrt()) as usize)))
        //     .collect();
        // println!("desired robots: {:?}", &robot_amount_goals);

        // let mut log = StepLog::new();
        // for _time_left in (0..total_time).rev() {
        //     state.step(
        //         &blueprint,
        //         |current_state, options| {
        //             let desired_craft = priority.iter().find_map(|robot_kind| {
        //                 match robot_amount_goals.get(robot_kind) {
        //                     Some(desired_amount) => match current_state.robots.get(robot_kind) {
        //                         Some(current_amount) => match current_amount.cmp(desired_amount) {
        //                             std::cmp::Ordering::Less => Some(robot_kind),
        //                             _ => None,
        //                         },
        //                         None => Some(robot_kind),
        //                     },
        //                     None => Some(robot_kind),
        //                 }
        //             })?;

        //             // println!(
        //             //     "Minute {}\n\tdesired_craft: {:?}",
        //             //     total_time - time_left,
        //             //     desired_craft
        //             // );

        //             let option = options.filter(|&option| option.eq(desired_craft)).next()?;

        //             // println!("\tCrafted");

        //             Some(option)
        //             //
        //             // strategy_priority_craft(options, current_state, &blueprint, time_left)
        //         },
        //         &mut log,
        //     );
        //     // println!(
        //     //     "minute {}:\n\testimations: {:?}\n\trobots made: {:?}",
        //     //     total_time - time_left,
        //     //     estimate_materials(&state, time_left),
        //     //     &state.robots
        //     // );
        // }
        // println!("{}", serde_json::to_string_pretty(&state).unwrap());
        //     println!("\nSimulation Results:");
        //     println!(
        //         "\trobots made: (total: {}) {:?}",
        //         &state
        //             .robots
        //             .iter()
        //             .map(|(_, amount)| *amount)
        //             .sum::<usize>(),
        //         &state.robots
        //     );
        //     println!(
        //         "\tmaterials left: (total: {}) {:?}",
        //         &state
        //             .minerals
        //             .iter()
        //             .map(|(_, amount)| *amount)
        //             .sum::<usize>(),
        //         &state.minerals
        //     );
        //     println!(
        //         "\tmaterials collected: (total: {}) {:?}",
        //         &log.materials_collected
        //             .iter()
        //             .map(|(_, amount)| *amount)
        //             .sum::<usize>(),
        //         &log.materials_collected
        //     );
        //     println!(
        //         "\tmaterials spent: (total: {}) {:?}",
        //         &log.materials_spent
        //             .iter()
        //             .map(|(_, amount)| *amount)
        //             .sum::<usize>(),
        //         &log.materials_spent
        //     );
        //     let quality = state
        //         .minerals
        //         .get(&Mineral::Geode)
        //         .map(|amount| *amount)
        //         .unwrap_or_default();
        //     quality_sum += index * quality;
        //     println!("Quality: {}", quality);

        //     println!();
        //     // break;
    }

    println!("------------------\nQuality sum = {}", quality_sum);
}

fn max_geodes(
    blueprint: &Blueprint<usize>,
    mut pack: Pack<usize>,
    time_left: usize,
    skip: &MineralArray<bool>,
    mut reached_optimal_cycle: bool,
) -> usize {
    if time_left == 0 {
        return pack.minerals[Mineral::Geode];
    }
    let mut crafts_to_try = MineralArray::default();
    reached_optimal_cycle = reached_optimal_cycle
        || blueprint[Mineral::Geode]
            .0
            .iter()
            .zip(pack.robots.0.iter())
            .all(|(desired, current)| desired >= current);
    match reached_optimal_cycle {
        true => {
            crafts_to_try[Mineral::Geode] = blueprint.can_craft(Mineral::Geode, &pack.minerals);
        }
        false => {
            for robot in Mineral::iter() {
                crafts_to_try[robot] = !skip[robot] & blueprint.can_craft(robot, &pack.minerals)
            }
        }
    }

    for mineral in Mineral::iter() {
        pack.minerals[mineral] += pack.robots[mineral];
    }

    Mineral::iter()
        .filter(|&robot| crafts_to_try[robot])
        .map(|robot| {
            let mut pack = pack.clone();
            for mineral in Mineral::iter() {
                pack.minerals[mineral] -= blueprint[robot][mineral];
            }
            pack.robots[robot] += 1;
            max_geodes(
                blueprint,
                pack,
                time_left - 1,
                &MineralArray::default(),
                reached_optimal_cycle,
            )
        })
        .chain([max_geodes(
            blueprint,
            pack.clone(),
            time_left - 1,
            &crafts_to_try,
            reached_optimal_cycle,
        )])
        .max()
        .unwrap()
}
