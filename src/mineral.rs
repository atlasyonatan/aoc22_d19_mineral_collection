use std::ops::{Index, IndexMut};

use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, EnumCount, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Mineral {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

#[derive(Debug, Clone)]
pub struct MineralArray<T>(pub [T; Mineral::COUNT]);

impl<T: Default> Default for MineralArray<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Index<Mineral> for MineralArray<T> {
    type Output = T;

    fn index(&self, index: Mineral) -> &Self::Output {
        self.0.index(index as usize)
    }
}

impl<T> IndexMut<Mineral> for MineralArray<T> {
    fn index_mut(&mut self, index: Mineral) -> &mut Self::Output {
        self.0.index_mut(index as usize)
    }
}

impl<T: Default> FromIterator<(Mineral, T)> for MineralArray<T> {
    fn from_iter<I: IntoIterator<Item = (Mineral, T)>>(iter: I) -> Self {
        let mut array = Self::default();
        for (mineral, item) in iter.into_iter() {
            array[mineral] = item;
        }
        array
    }
}
