use crate::domain::aggregate::{Plane, Position, Status};
use crate::domain::events::Event;

use cqrs_es::persist::GenericQuery;
use cqrs_es::{EventEnvelope, View};
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CurrentJourneyView {
    registration_id: String,
    status: Status,
    positions: Vec<Position>,
}

pub type CurrentJourneyQuery =
    GenericQuery<PostgresViewRepository<CurrentJourneyView, Plane>, CurrentJourneyView, Plane>;

impl View<Plane> for CurrentJourneyView {
    fn update(&mut self, event: &EventEnvelope<Plane>) {
        match &event.payload {
            Event::Registered { registration_id } => self.registration_id = registration_id.clone(),
            Event::OnGround => self.status = Status::OnGround,
            Event::TookOff => {
                self.status = Status::InAir;
                self.positions.clear();
            }
            Event::Landed => self.status = Status::OnGround,
            Event::PositionedAt {
                latitude,
                longitude,
                altitude,
            } => self.positions.push(Position {
                latitude: *latitude,
                longitude: *longitude,
                altitude: *altitude,
            }),
        }
    }
}
