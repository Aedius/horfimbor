use crate::error::InvalidResources;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resources {
    uranium: usize,
    steel: usize,
    gold: usize,

}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Buildings {
    house: usize,
    mine: usize,
}

impl Resources {
    pub fn new(uranium: usize, steel: usize) -> Result<Resources, InvalidResources> {
        if uranium + steel > 100000 {
            Err(InvalidResources)
        } else {
            Ok(Resources {
                uranium,
                steel,
                gold: 100000 - uranium - steel,
            })
        }
    }
}