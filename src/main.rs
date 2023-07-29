pub mod blueprint;
pub mod mineral;
pub mod pack;

use crate::blueprint::Blueprint;
use crate::mineral::Mineral;
use crate::pack::Pack;

use itertools::Itertools;
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
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let pack = {
        let mut pack = Pack::default();
        pack.robots[Mineral::Ore] = 1;
        pack
    };
    let goal = Mineral::Geode;

    let time = 24;
    let quality_sum = blueprints
        .iter()
        .enumerate()
        .map(|(index, blueprint)| {
            (index + 1) * solve_max_mineral(goal, &blueprint, pack.clone(), time)
        })
        .sum::<usize>();
    println!("Part 1: Quality sum = {}", quality_sum);

    // let time = 32;
    // let product = blueprints
    //     .iter()
    //     .take(3)
    //     .map(|blueprint| {
    //         let max = solve_max_mineral(goal, &blueprint, pack.clone(), time);
    //         println!("max: {}\n\tblueprint: {:?}",&max, &blueprint);
    //         max
    //     })
    //     .product::<usize>();

    // println!("Part 2: Product = {}", product);
}

fn solve_max_mineral(
    goal: Mineral,
    blueprint: &Blueprint<usize>,
    pack: Pack<usize>,
    time: usize,
) -> usize {
    let max = Mineral::iter()
        .map(|robot| &blueprint[robot])
        .flat_map(|recipie| Mineral::iter().map(|mineral| (mineral, recipie[mineral])))
        .into_grouping_map()
        .max()
        .into_iter()
        .collect::<MineralArray<_>>();

    max_mineral(
        goal,
        blueprint,
        pack,
        time,
        &MineralArray::default(),
        &max,
        0,
    )
}

fn max_mineral(
    goal: Mineral,
    blueprint: &Blueprint<usize>,
    mut pack: Pack<usize>,
    remaining_time: usize,
    skip_crafts: &MineralArray<bool>,
    max_robots_needed: &MineralArray<usize>,
    mut max_result: usize,
) -> usize {
    if remaining_time == 0 {
        return pack.minerals[goal];
    }

    let upperbound =
        pack.minerals[goal] + remaining_time * (pack.robots[goal] + (remaining_time - 1) / 2);

    if upperbound < max_result {
        return 0;
    }

    let available_crafts = Mineral::iter()
        .map(|robot| (robot, blueprint.can_craft(robot, &pack.minerals)))
        .collect::<MineralArray<_>>();

    for mineral in Mineral::iter() {
        pack.minerals[mineral] += pack.robots[mineral];
    }

    let crafts = Mineral::iter()
        .filter(|&robot| available_crafts[robot])
        .filter(|&robot| !skip_crafts[robot])
        .filter(|&robot| robot == goal || pack.robots[robot] < max_robots_needed[robot]);

    for robot in crafts {
        let mut pack = pack.clone();
        for mineral in Mineral::iter() {
            pack.minerals[mineral] -= blueprint[robot][mineral];
        }
        pack.robots[robot] += 1;
        max_result = max_result.max(max_mineral(
            goal,
            blueprint,
            pack,
            remaining_time - 1,
            &MineralArray::default(),
            max_robots_needed,
            max_result,
        ))
    }

    let max_result = max_result.max(max_mineral(
        goal,
        blueprint,
        pack,
        remaining_time - 1,
        &available_crafts,
        max_robots_needed,
        max_result,
    ));

    max_result
}
