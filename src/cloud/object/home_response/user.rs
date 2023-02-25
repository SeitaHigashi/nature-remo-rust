use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    joind_at: String,
    location_state: String,
    nickname: String,
    role: String,
}
