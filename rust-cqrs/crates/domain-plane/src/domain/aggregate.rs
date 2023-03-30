use crate::domain::aggregate::Status::OnGround;
use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use crate::domain::commands::Command;
use crate::domain::errors::Error;
use crate::domain::events::Event;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: usize,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub enum Status {
    #[default]
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
            Command::Create { registration_id } => {
                if self.registration_id != "" {
                    return Err(Error::AlreadyCreated);
                }
                Ok(vec![Event::Created { registration_id }, Event::OnGround])
            }
            Command::UpdatePosition {
                latitude,
                longitude,
                altitude,
            } => Ok(vec![Event::PositionedAt {
                // TODO: should validate
                latitude,
                longitude,
                altitude,
            }]),
            Command::TakeOff => {
                if self.status == Status::OnGround {
                    // TODO: call TowerControl service to ensure we can takeoff
                    Ok(vec![Event::TookOff])
                } else {
                    Err(Error::CannotTakeOff)
                }
            }
            Command::Land => {
                if self.status == Status::InAir {
                    // TODO: call TowerControl service to ensure we can land
                    Ok(vec![Event::Landed])
                } else {
                    Err(Error::CannotLand)
                }
            }
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
            Event::TookOff => self.status = Status::InAir,
            Event::Landed => self.status = Status::OnGround,
            Event::OnGround => self.status = Status::OnGround,
        }
    }
}
