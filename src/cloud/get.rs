use super::api::API;

pub enum GET {
    Appliances,
    Signals{applianceid: String},
    Devices,
    Homes,
    HomeDevices{homeid: String},
    HomeUsers{homeid: String},
    Invites{invitetoken: String},
    Users,
}

impl API for GET {
    fn path(&self) -> String {
        match self {
            Self::Appliances => String::from("/1/appliances"),
            Self::Signals{ applianceid } => format!("/1/appliances/{}/signals/", applianceid),
            Self::Devices => String::from("/1/devices"),
            Self::Homes => String::from("/1/homes"),
            Self::HomeDevices { homeid } => format!("/1/homes/{}/devices", homeid),
            Self::HomeUsers{ homeid } => format!("/1/homes/{}/users", homeid),
            Self::Invites{ invitetoken } => format!("/1/invites/{}", invitetoken),
            Self::Users => String::from("/1/users/me"),
        }
    }
}

