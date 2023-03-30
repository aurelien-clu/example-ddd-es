use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Event {
    Registered {
        registration_id: String,
    },
    OnGround,
    TookOff,
    Landed,
    PositionedAt {
        latitude: f64,
        longitude: f64,
        altitude: usize,
    },
}

impl DomainEvent for Event {
    fn event_type(&self) -> String {
        match self {
            Event::Registered { .. } => "Registered".to_string(),
            Event::OnGround { .. } => "OnGround".to_string(),
            Event::TookOff { .. } => "TookOff".to_string(),
            Event::Landed { .. } => "Landed".to_string(),
            Event::PositionedAt { .. } => "PositionedAt".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
