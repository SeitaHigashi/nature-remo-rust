pub mod location;
pub mod user;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct HomeResponse {
    id: String,
    location: location::Location,
    name: String,
    usres: Vec<user::User>,
}
