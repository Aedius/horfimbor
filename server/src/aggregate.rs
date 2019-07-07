use cqrs_core::{
    Aggregate, AggregateEvent, AggregateId, DeserializableEvent, Event, SerializableEvent,
};
use serde::{Deserialize, Serialize};
use crate::domain;
use crate::event;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanetAggregate {

    Created(PlanetData),

    /// An uninitialized to-do item.
    Uninitialized,
}

impl Default for PlanetAggregate {
    fn default() -> Self {
        PlanetAggregate::Uninitialized
    }
}

impl PlanetAggregate {

    pub fn get_data(&self) -> Option<&PlanetData> {
        match *self {
            PlanetAggregate::Uninitialized => None,
            PlanetAggregate::Created(ref x) => Some(x),
        }
    }
}

impl Aggregate for PlanetAggregate {
    #[inline(always)]
    fn aggregate_type() -> &'static str
        where
            Self: Sized,
    {
        "planet"
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PlanetId(pub String);

impl AggregateId<PlanetAggregate> for PlanetId {
    fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct PlanetIdRef<'a>(pub &'a str);

impl<'a> AsRef<str> for PlanetIdRef<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl<'a> AggregateId<PlanetAggregate> for PlanetIdRef<'a> {
    fn as_str(&self) -> &str {
        self.0
    }
}

pub struct PlanetMetadata {
    /// The actor that caused this event to be added to the event stream.
    pub initiated_by: String,
}


/// Data relating to a to-do item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanetData {
    pub resources: domain::Resources,
    pub buildings: Option<domain::Buildings>,

    /// The current status of this item.
    pub status: PlanetStatus,
}

impl PlanetData {
    fn with_resources(resources: domain::Resources) -> Self {

        PlanetData {
            resources,
            buildings: None,
            status: PlanetStatus::Empty,
        }
    }
}

/// The completion status of a to-do item.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanetStatus {
    Build,
    Empty,
}

pub enum PlanetEvent{
    Created(event::Created),
    Constructed(event::Constructed),
}

impl Event for PlanetEvent {
    fn event_type(&self) -> &'static str {
        match *self{
            PlanetEvent::Created(ref evt) => evt.event_type(),
            PlanetEvent::Constructed(ref evt) => evt.event_type(),
        }
    }
}

impl AggregateEvent<PlanetAggregate> for event::Created {
    fn apply_to(self, aggregate: &mut PlanetAggregate) {
        if PlanetAggregate::Uninitialized == *aggregate {
            *aggregate =
                PlanetAggregate::Created(PlanetData::with_resources(self.initial_resources))
        }
    }
}
impl AggregateEvent<PlanetAggregate> for event::Constructed {
    fn apply_to(self, aggregate: &mut PlanetAggregate) {
        if let PlanetAggregate::Created(ref mut data) = aggregate{
             // FIXME
            data.resources = self.used_resources;
            data.buildings = Some(self.new_building);
        }
    }
}

impl SerializableEvent for PlanetEvent{
    type Error = serde_json::Error;

    fn serialize_event_to_buffer(&self, buffer: &mut Vec<u8>) -> Result<(), Self::Error> {
        buffer.clear();
        buffer.reserve(128);
        match *self {
            PlanetEvent::Created(ref inner) => {
                serde_json::to_writer(buffer, inner)?;
            }
            PlanetEvent::Constructed(ref inner) => {
                serde_json::to_writer(buffer, inner)?;
            }
        }
        Ok(())
    }
}

impl DeserializableEvent for PlanetEvent {
    type Error = serde_json::Error;

    fn deserialize_event_from_buffer(
        data: &[u8],
        event_type: &str,
    ) -> Result<Option<Self>, Self::Error> {
        let deserialized = match event_type {
            "todo_created" => PlanetEvent::Created(serde_json::from_slice(data)?),
            "todo_constructed" => PlanetEvent::Constructed(serde_json::from_slice(data)?),
            _ => return Ok(None),
        };
        Ok(Some(deserialized))
    }
}