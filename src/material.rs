use serde::Serialize;
use strum_macros::{self, EnumString};
#[derive(EnumString, Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize)]
#[strum(serialize_all = "lowercase")]
pub enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
