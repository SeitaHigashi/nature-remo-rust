use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Location {
    latitude: f64,
    longitude: f64,
    radius: f32,
}
