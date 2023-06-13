use lazy_static::lazy_static;
use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

pub type Blueprint = super::Blueprint<String, Vec<(usize, String)>>;

impl FromStr for Blueprint {
    type Err = ParseError;

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
                    let robot_kind = caps.get(1).ok_or(MissingCaptureError)?.as_str().to_string();
                    let robot_costs = caps.get(2).ok_or(MissingCaptureError)?.as_str();
                    let robot_costs = COSTS_REG
                        .captures_iter(robot_costs)
                        .map(|caps| {
                            let amount = caps
                                .get(1)
                                .ok_or(MissingCaptureError)?
                                .as_str()
                                .parse::<usize>()?;
                            let material =
                                caps.get(2).ok_or(MissingCaptureError)?.as_str().to_string();
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
pub enum ParseError {
    #[error("{0}")]
    AmountParseError(#[from] ParseIntError),
    #[error("{0:?}")]
    MissingCaptureError(#[from] MissingCaptureError),
}
