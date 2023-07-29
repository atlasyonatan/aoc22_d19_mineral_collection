use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use strum::IntoEnumIterator;
use thiserror::Error;

use crate::mineral::{Mineral, MineralArray};

pub type Blueprint<Amount> = MineralArray<MineralArray<Amount>>;

#[derive(Error, Debug)]
pub enum ParseError<MineralParseError, AmountParseError> {
    #[error("{0}")]
    MineralParseError(MineralParseError),
    #[error("{0}")]
    AmountParseError(AmountParseError),
    #[error("MissingCaptureError")]
    MissingCaptureError,
}

impl<Amount> FromStr for Blueprint<Amount>
where
    Amount: FromStr + Default,
{
    type Err = ParseError<<Mineral as FromStr>::Err, Amount::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BLUEPRINT_REG: Regex =
                Regex::new(r"Each (\w+) robot costs (.*?)\.").unwrap();
            static ref COSTS_REG: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
        }
        let mut blueprint = Self::default();
        for caps in BLUEPRINT_REG.captures_iter(s) {
            let robot_mineral = caps
                .get(1)
                .ok_or(Self::Err::MissingCaptureError)?
                .as_str()
                .parse::<Mineral>()
                .map_err(|err| Self::Err::MineralParseError(err))?;
            let robot_costs_str = caps.get(2).ok_or(Self::Err::MissingCaptureError)?.as_str();
            let mut robot_costs = MineralArray::default();
            for caps in COSTS_REG.captures_iter(robot_costs_str) {
                let amount = caps
                    .get(1)
                    .ok_or(Self::Err::MissingCaptureError)?
                    .as_str()
                    .parse::<Amount>()
                    .map_err(|err| Self::Err::AmountParseError(err))?;
                let mineral = caps
                    .get(2)
                    .ok_or(Self::Err::MissingCaptureError)?
                    .as_str()
                    .parse::<Mineral>()
                    .map_err(|err| Self::Err::MineralParseError(err))?;
                robot_costs[mineral] = amount
            }
            blueprint[robot_mineral] = robot_costs
        }
        Ok(blueprint)
    }
}

impl<Amount: PartialOrd> Blueprint<Amount> {
    pub fn can_craft(&self, robot: Mineral, inventory: &MineralArray<Amount>) -> bool {
        self[robot]
            .0
            .iter()
            .zip(inventory.0.iter())
            .all(|(required, current)| required <= current)
    }
}

impl<Amount: Ord + Default + Copy> Blueprint<Amount> {
    pub fn max_robots_needed(&self) -> MineralArray<Amount> {
        Mineral::iter()
            .map(|robot| &self[robot])
            .flat_map(|recipie| Mineral::iter().map(|mineral| (mineral, recipie[mineral])))
            .into_grouping_map()
            .max()
            .into_iter()
            .collect()
    }
}
