use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ACSmartMode {
    adjusting: bool,
    area: u8,
    enabled: bool,
}
