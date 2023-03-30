use crate::domain::aggregate::Status::OnGround;
use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use crate::domain::commands::Command;
use crate::domain::errors::Error;
use crate::domain::events::Event;

#[derive(Serialize, Deserialize)]
pub struct Position {
    latitude: f64,
    longitude: f64,
    altitude: usize,
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    OnGround,
    InAir,
}

#[derive(Serialize, Deserialize)]
pub struct Plane {
    registration_id: String, // https://en.wikipedia.org/wiki/Aircraft_registration
    last_position: Option<Position>,
    status: Status,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            registration_id: "".to_string(),
            last_position: None,
            status: OnGround,
        }
    }
}

#[async_trait]
impl Aggregate for Plane {
    type Command = Command;
    type Event = Event;
    type Error = Error;
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
            Command::Create { .. } => {
                if self.registration_id != "" {
                    return Err(Error::AlreadyCreated);
                }
                Ok(vec![])
            }
            Command::UpdatePosition { .. } => todo!(),
            Command::TakeOff { .. } => todo!(),
            Command::Land { .. } => todo!(),
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Event::Created { registration_id } => self.registration_id = registration_id,
            Event::PositionedAt {
                latitude,
                longitude,
                altitude,
            } => {
                let p = Position {
                    latitude,
                    longitude,
                    altitude,
                };
                self.last_position = Some(p);
            }
            Event::TookOff { .. } => todo!(),
            Event::Landed { .. } => todo!(),
            Event::OnGround { .. } => todo!(),
        }
    }
}
