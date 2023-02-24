use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Range {
    fixed_buttons: Vec<String>,
}
