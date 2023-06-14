use serde::Serialize;
use strum_macros::{self, EnumString, Display};
#[derive(EnumString, Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
