use async_trait::async_trait;
use cqrs_es::{EventEnvelope, Query};

use crate::domain::aggregate::Plane;

pub struct LoggingQuery {}

#[async_trait]
impl Query<Plane> for LoggingQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<Plane>]) {
        for event in events {
            let payload = serde_json::to_string_pretty(&event.payload).unwrap();
            println!("******************************************************");
            println!("id: '{}', sequence: {}", aggregate_id, event.sequence);
            println!("{}", payload);
        }
    }
}
