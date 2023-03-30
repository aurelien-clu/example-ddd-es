use async_trait::async_trait;
use cqrs_es::persist::GenericQuery;
use cqrs_es::{EventEnvelope, Query, View};
use postgres_es::PostgresViewRepository;
use serde::{Deserialize, Serialize};

use crate::domain::aggregate::Plane;
use crate::domain::events::Event;

pub struct LoggingQuery {}

#[async_trait]
impl Query<Plane> for LoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Plane>]) {
        for event in events {
            let payload = serde_json::to_string(&event.payload).unwrap();
            println!("{}-{}\n{}", aggregate_id, event.sequence, payload);
        }
    }
}
