pub mod home;

use home::*;
use super::api::API;

pub enum GET {
    Appliances,
    Signals{applianceid: String},
    Devices,
    Homes{home: Option<Home>},
    Invites{invitetoken: String},
    Users,
}

impl API for GET {
    fn path(&self) -> String {
        match self {
            GET::Appliances => String::from("/1/appliances"),
            GET::Signals{ applianceid } => format!("/1/appliances/{}/signals/", applianceid),
            GET::Devices => String::from("/1/devices"),
            GET::Homes{ home: Option::None }=> String::from("/1/homes"),
            GET::Homes{ home: Option::Some(home) }=> format!("/1/homes{}", home.path()),
            GET::Invites{ invitetoken } => format!("/1/invites/{}", invitetoken),
            GET::Users => String::from("/1/users/me"),
        }
    }
}

