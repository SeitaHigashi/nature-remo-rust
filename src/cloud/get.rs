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
            GET::Appliances => String::from("/1/appliances"),
            GET::Signals{ applianceid } => format!("/1/appliances/{}/signals/", applianceid),
            GET::Devices => String::from("/1/devices"),
            GET::Homes => String::from("/1/homes"),
            GET::HomeDevices { homeid } => format!("/1/homes/{}/devices", homeid),
            GET::HomeUsers{ homeid } => format!("/1/homes/{}/users", homeid),
            GET::Invites{ invitetoken } => format!("/1/invites/{}", invitetoken),
            GET::Users => String::from("/1/users/me"),
        }
    }
}

