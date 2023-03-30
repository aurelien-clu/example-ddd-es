use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    Create {
        registration_id: String,
    },
    UpdatePosition {
        latitude: f64,
        longitude: f64,
        altitude: usize,
    },
    TakeOff,
    Land,
}
