use std::collections::HashMap;

#[derive(Debug)]
pub struct Blueprint<Kind, Cost> {
    catalogue: HashMap<Kind, Cost>,
}

pub mod parse;
