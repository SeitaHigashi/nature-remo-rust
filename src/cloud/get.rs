use super::api::API;

pub enum GET {
    Appliances,
    Signals{applianceid: String},
    Devices,
    Homes,
    HomeDevices{homeid: String},
    Users{homeid: String},
    Invites{invitetoken: String},
    Me,
}

impl API for GET {
    fn path(&self) -> String {
        match self {
            Self::Appliances => String::from("/1/appliances"),
            Self::Signals{ applianceid } => format!("/1/appliances/{}/signals/", applianceid),
            Self::Devices => String::from("/1/devices"),
            Self::Homes => String::from("/1/homes"),
            Self::HomeDevices { homeid } => format!("/1/homes/{}/devices", homeid),
            Self::Users{ homeid } => format!("/1/homes/{}/users", homeid),
            Self::Invites{ invitetoken } => format!("/1/invites/{}", invitetoken),
            Self::Me => String::from("/1/users/me"),
        }
    }
}
