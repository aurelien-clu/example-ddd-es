use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use crate::domain::commands::PlaneCommand;
use crate::domain::errors::PlaneError;
use crate::domain::events::PlaneEvent;

#[derive(Serialize, Deserialize)]
pub struct Position {
    latitude: f64,
    longitude: f64,
    altitude: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Plane {
    // https://en.wikipedia.org/wiki/Aircraft_registration
    registration_id: String,
    last_position: Option<Position>,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            registration_id: "".to_string(),
            last_position: None,
        }
    }
}

#[async_trait]
impl Aggregate for Plane {
    type Command = PlaneCommand;
    type Event = PlaneEvent;
    type Error = PlaneError;
    type Services = ();

    fn aggregate_type() -> String {
        "plane".to_string()
    }

    async fn handle(
        &self,
        command: Self::Command,
        services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            PlaneCommand::Create { .. } => todo!(),
            PlaneCommand::UpdatePosition { .. } => todo!(),
            PlaneCommand::TakeOff { .. } => todo!(),
            PlaneCommand::Land { .. } => todo!(),
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            PlaneEvent::Created { .. } => todo!(),
            PlaneEvent::PositionedAt { .. } => todo!(),
            PlaneEvent::TookOff { .. } => todo!(),
            PlaneEvent::Landed { .. } => todo!(),
            PlaneEvent::OnGround { .. } => todo!(),
        }
    }
}
