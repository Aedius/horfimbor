
use crate::domain;
use cqrs_core::Event;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Created {
    pub initial_resources: domain::Resources,
}

impl Event for Created {
    fn event_type(&self) -> &'static str {
        "planet_created"
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Constructed {
    pub used_resources: domain::Resources,
    pub new_building: domain::Buildings,
}

impl Event for Constructed {
    fn event_type(&self) -> &'static str {
        "planet_constructed"
    }
}