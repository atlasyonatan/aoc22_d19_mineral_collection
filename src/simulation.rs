use crate::blueprint::MaterialBlueprint;
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[derive(Debug, Serialize)]
pub struct State<Material, Amount>
where
    Material: Hash + Eq,
{
    pub materials: HashMap<Material, Amount>,
    pub robots: HashMap<Material, Amount>,
}

impl<Material: Hash + Eq> State<Material, usize> {
    pub fn new() -> Self {
        Self {
            materials: HashMap::new(),
            robots: HashMap::new(),
        }
    }

    pub fn step<F>(
        &mut self,
        blueprint: &MaterialBlueprint<Material, usize>,
        get_instruction: F,
    ) -> ()
    where
        Material: Hash + Eq + Clone + Copy + Debug,
        F: for<'a> Fn(&Self, &mut dyn Iterator<Item = &'a Material>) -> Option<&'a Material>,
    {
        //create robot
        let instruction =
            get_instruction(&self, &mut self.available_options(&blueprint)).map(|choise| *choise);
        if let Some(instruction_material) = &instruction {
            let recipie = blueprint.0.get(instruction_material).unwrap();
            for (amount, material) in recipie {
                *self.materials.get_mut(material).unwrap() -= amount
            }
        }

        //robots collect
        for (material, amount) in self.robots.iter() {
            *self.materials.entry(*material).or_default() += amount;
        }

        //robot created
        if let Some(material) = &instruction {
            *self.robots.entry(*material).or_default() += 1;
        }
    }

    pub fn available_options<'a>(
        &'a self,
        blueprint: &'a MaterialBlueprint<Material, usize>,
    ) -> impl Iterator<Item = &'a Material> {
        blueprint.0.iter().filter_map(|(robot, cost)| {
            cost.iter()
                .all(|(required_amount, material)| {
                    self.materials
                        .get(material)
                        .is_some_and(|current_amount| current_amount >= required_amount)
                })
                .then_some(robot)
        })
    }
}
