pub mod ac_smart_mode;
pub mod aircon;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ApplianceResponse {
    ac_smart_mode: ac_smart_mode::ACSmartMode,
    id: String,
    image: String,
    nickname: String,
    appliance_type: String,
}
