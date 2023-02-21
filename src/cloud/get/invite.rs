pub enum Invite {
    None{invitetoken: String},
}

impl Invite {
    pub fn path(&self) -> String {
        match self {
            Invite::None { invitetoken } => format!("/{}", invitetoken),
        }
    }
}
