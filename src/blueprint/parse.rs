use lazy_static::lazy_static;
use regex::Regex;
use std::{hash::Hash, num::ParseIntError, str::FromStr};
use thiserror::Error;

pub type Blueprint<Material> = super::Blueprint<Material, Vec<(usize, Material)>>;

impl<Material: Eq + Hash + FromStr> FromStr for Blueprint<Material> {
    type Err = ParseError<Material::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BLUEPRINT_REG: Regex =
                Regex::new(r"Each (\w+) robot costs (.*?)\.").unwrap();
            static ref COSTS_REG: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
        }
        Ok(Blueprint {
            catalogue: BLUEPRINT_REG
                .captures_iter(s)
                .map(|caps| {
                    let robot_kind = caps
                        .get(1)
                        .ok_or(MissingCaptureError)?
                        .as_str()
                        .parse::<Material>()
                        .map_err(|err| Self::Err::MaterialParseError(err))?;
                    let robot_costs = caps.get(2).ok_or(MissingCaptureError)?.as_str();
                    let robot_costs = COSTS_REG
                        .captures_iter(robot_costs)
                        .map(|caps| {
                            let amount = caps
                                .get(1)
                                .ok_or(MissingCaptureError)?
                                .as_str()
                                .parse::<usize>()?;
                            let material = caps
                                .get(2)
                                .ok_or(MissingCaptureError)?
                                .as_str()
                                .parse::<Material>()
                                .map_err(|err| Self::Err::MaterialParseError(err))?;
                            Ok((amount, material))
                        })
                        .collect::<Result<Vec<_>, Self::Err>>()?;
                    Ok((robot_kind, robot_costs))
                })
                .collect::<Result<_, Self::Err>>()?,
        })
    }
}

#[derive(Error, Debug)]
#[error("MissingCaptureError")]
pub struct MissingCaptureError;

#[derive(Error, Debug)]
pub enum ParseError<MaterialParseError> {
    #[error("{0}")]
    AmountParseError(#[from] ParseIntError),
    #[error("{0}")]
    MissingCaptureError(#[from] MissingCaptureError),
    #[error("{0}")]
    MaterialParseError(MaterialParseError),
}
