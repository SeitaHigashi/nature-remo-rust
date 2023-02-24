pub mod range;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Aircon {
    range: range::Range,
    temp_unit: String,
}
