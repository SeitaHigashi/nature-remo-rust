pub enum Home {
    None{homeid: Option<String>},
    Devices{homeid: String},
    Users{homeid: String},
}

impl Home {
    pub fn path(&self) -> String {
        match self {
            Home::None { homeid: Option::None } => String::from(""),
            Home::None { homeid: Some(homeid) } => format!("/{}", homeid),
            Home::Devices { homeid } => format!("/{}/devices", homeid),
            Home::Users { homeid } => format!("/{}/users", homeid),
        }
    }
}
