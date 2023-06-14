use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimulationError<Material, Amount> {
    #[error("Instruction: [{instruction}] resulted in an error: {error}")]
    InstcurionError {
        instruction: Material,
        error: InstcurionError<Material, Amount>,
    },
}

#[derive(Error, Debug)]
#[error("Required {required} [{material}] but had {in_stock}")]
pub struct InsufficientMaterialError<Material, Amount> {
    pub material: Material,
    pub in_stock: Amount,
    pub required: Amount,
}

#[derive(Debug)]
pub enum InstcurionError<Material, Amount> {
    InsufficientMaterialsError(Vec<InsufficientMaterialError<Material, Amount>>),
}

impl<Material: Display, Amount: Display> Display for InstcurionError<Material, Amount> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstcurionError::InsufficientMaterialsError(errors) => {
                let s = errors
                    .iter()
                    .map(|error| format!("{}", error))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}.", s)?;
            }
        };
        Ok(())
    }
}
