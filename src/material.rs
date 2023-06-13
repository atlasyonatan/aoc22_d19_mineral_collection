use serde::Serialize;
use strum_macros::{self, EnumIter, EnumString};
#[derive(EnumString, EnumIter, Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize)]
#[strum(serialize_all = "lowercase")]
pub enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
