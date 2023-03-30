use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PlaneCommand {
    Create {
        registration_code: String,
    },
    UpdatePosition {
        latitude: f64,
        longitude: f64,
        altitude: usize,
    },
    TakeOff,
    Land,
}
