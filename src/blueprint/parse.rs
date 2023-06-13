use lazy_static::lazy_static;
use regex::Regex;
use std::{hash::Hash, str::FromStr};
use thiserror::Error;

pub type Blueprint<Material, Amount> = super::Blueprint<Material, Vec<(Amount, Material)>>;

impl<Material, Amount> FromStr for Blueprint<Material, Amount>
where
    Material: Eq + Hash + FromStr,
    Amount: FromStr,
{
    type Err = ParseError<Material::Err, Amount::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BLUEPRINT_REG: Regex =
                Regex::new(r"Each (\w+) robot costs (.*?)\.").unwrap();
            static ref COSTS_REG: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
        }
        Ok(Self {
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
                                .parse::<Amount>()
                                .map_err(|err| Self::Err::AmountParseError(err))?;
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
pub enum ParseError<MaterialParseError, AmountParseError> {
    #[error("{0}")]
    AmountParseError(AmountParseError),
    #[error("{0}")]
    MissingCaptureError(#[from] MissingCaptureError),
    #[error("{0}")]
    MaterialParseError(MaterialParseError),
}
