pub mod error;
use crate::{
    blueprint::MaterialBlueprint,
    simulation::error::{InstcurionError, InsufficientMaterialError, SimulationError},
};
use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[derive(Debug)]
pub struct State<Material, Amount>
where
    Material: Hash + Eq,
{
    pub materials_collected: HashMap<Material, Amount>,
    pub active_robots: HashMap<Material, Amount>,
}

pub fn simulate<Material, F>(
    blueprint: &MaterialBlueprint<Material, usize>,
    time: usize,
    get_instruction: F,
) -> Result<State<Material, usize>, SimulationError<Material, usize>>
where
    Material: Hash + Eq + Clone + Copy + Debug,
    F: Fn(&MaterialBlueprint<Material, usize>, &State<Material, usize>) -> Option<Material>,
{
    let map = HashMap::from_iter(blueprint.0.keys().map(|&material| (material, 0)));
    let mut state = State {
        active_robots: map.clone(),
        materials_collected: map,
    };
    for _ in 0..time {
        //create robot
        let instruction = get_instruction(blueprint, &state);
        if let Some(instruction_material) = &instruction {
            let recipie = blueprint.0.get(instruction_material).unwrap();
            let (leftovers, errors) = recipie
                .iter()
                .map(|(amount, material)| {
                    let stock = state.materials_collected.get_mut(material).unwrap();
                    let leftover =
                        stock
                            .checked_sub(*amount)
                            .ok_or_else(|| InsufficientMaterialError {
                                material: *material,
                                in_stock: *stock,
                                required: *amount,
                            })?;
                    Ok((leftover, material))
                })
                .partition::<Vec<_>, _>(Result::is_ok);
            if errors.len() > 0 {
                return Err(SimulationError::InstcurionError {
                    instruction: *instruction_material,
                    error: InstcurionError::InsufficientMaterialsError(
                        errors.into_iter().map(Result::unwrap_err).collect(),
                    ),
                });
            }
            let leftovers: Vec<_> = leftovers.into_iter().map(Result::unwrap).collect();
            assert!(leftovers.len() == recipie.len());
            for (leftover, material) in leftovers.into_iter() {
                *state.materials_collected.get_mut(material).unwrap() = leftover
            }
        }

        //robots collect
        for (material, amount) in state.active_robots.iter() {
            *state.materials_collected.get_mut(&material).unwrap() += amount;
        }

        //robot created
        if let Some(material) = instruction {
            *state.active_robots.get_mut(&material).unwrap() += 1;
        }
    }
    Ok(state)
}
