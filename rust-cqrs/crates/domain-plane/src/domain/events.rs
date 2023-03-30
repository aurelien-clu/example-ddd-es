use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlaneEvent {
    Created,
    OnGround,
    TookOff,
    Landed,
    PositionedAt {
        latitude: f64,
        longitude: f64,
        altitude: usize,
    },
}

impl DomainEvent for PlaneEvent {
    fn event_type(&self) -> String {
        match self {
            PlaneEvent::Created { .. } => "OnGround".to_string(),
            PlaneEvent::OnGround { .. } => "OnGround".to_string(),
            PlaneEvent::TookOff { .. } => "TookOff".to_string(),
            PlaneEvent::Landed { .. } => "Landed".to_string(),
            PlaneEvent::PositionedAt { .. } => "PositionedAt".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
