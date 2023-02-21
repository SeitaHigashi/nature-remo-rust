pub enum Appliance {
    Signals{applianceid: String},
}

impl Appliance {
    pub fn path(&self) -> String {
        match self {
            Appliance::Signals { applianceid } => format!("/{}/signals", applianceid),
        }
    }
}
