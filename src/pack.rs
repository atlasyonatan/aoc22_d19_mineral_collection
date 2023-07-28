use crate::mineral::MineralArray;

#[derive(Debug, Clone)]
pub struct Pack<Amount> {
    pub minerals: MineralArray<Amount>,
    pub robots: MineralArray<Amount>,
}

impl<Amount: Default + Copy> Default for Pack<Amount> {
    fn default() -> Self {
        Self {
            minerals: Default::default(),
            robots: Default::default(),
        }
    }
}
