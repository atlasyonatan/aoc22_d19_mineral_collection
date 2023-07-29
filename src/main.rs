pub mod blueprint;
pub mod mineral;
pub mod pack;

use crate::{
    blueprint::Blueprint,
    mineral::{Mineral, MineralArray},
    pack::Pack,
};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use strum::IntoEnumIterator;

fn main() -> () {
    let file_path = "../input.txt";
    let path = Path::new(file_path);
    let file = File::open(path).unwrap();
    let blueprints = io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse::<Blueprint<usize>>())
        .map(Result::unwrap)
        .map(|blueprint| {
            let robots_needed = blueprint.max_robots_needed();
            (blueprint, robots_needed)
        })
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
        .map(|(blueprint, max_robots_needed)| {
            max_mineral(
                goal,
                &blueprint,
                max_robots_needed,
                time,
                pack.clone(),
                &Default::default(),
                0,
            )
        })
        .enumerate()
        .map(|(index, max_goal)| (index + 1) * max_goal)
        .sum::<usize>();
    println!("Part 1: Quality sum = {}", quality_sum);

    let time = 32;
    let product = blueprints
        .iter()
        .take(3)
        .map(|(blueprint, max_robots_needed)| {
            max_mineral(
                goal,
                &blueprint,
                max_robots_needed,
                time,
                pack.clone(),
                &Default::default(),
                0,
            )
        })
        .product::<usize>();

    println!("Part 2: Product = {}", product);
}

fn max_mineral(
    goal: Mineral,
    blueprint: &Blueprint<usize>,
    max_robots_needed: &MineralArray<usize>,
    remaining_time: usize,
    mut pack: Pack<usize>,
    skip_crafts: &MineralArray<bool>,
    mut max_result: usize,
) -> usize {
    if remaining_time == 0 {
        return pack.minerals[goal];
    }

    let upperbound = pack.minerals[goal]
        + remaining_time * pack.robots[goal]
        + remaining_time * (remaining_time - 1) / 2;

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
            max_robots_needed,
            remaining_time - 1,
            pack,
            &MineralArray::default(),
            max_result,
        ))
    }

    let max_result = max_result.max(max_mineral(
        goal,
        blueprint,
        max_robots_needed,
        remaining_time - 1,
        pack,
        &available_crafts,
        max_result,
    ));

    max_result
}
