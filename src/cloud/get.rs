pub mod home;
pub mod appliance;
pub mod invite;

use home::*;
use appliance::*;
use invite::*;
use super::api::API;

pub enum GET {
    Appliances{appliance: Option<Appliance>},
    Devices,
    Homes{home: Option<Home>},
    Invites{invitetoken: Invite},
    Users,
}

impl API for GET {
    fn path(&self) -> String {
        match self {
            GET::Appliances{ appliance: Option::None } => String::from("/1/appliances"),
            GET::Appliances{ appliance: Option::Some(appliance) } => format!("/1/appliances{}", appliance.path()),
            GET::Devices => String::from("/1/devices"),
            GET::Homes{ home: Option::None }=> String::from("/1/homes"),
            GET::Homes{ home: Option::Some(home) }=> format!("/1/homes{}", home.path()),
            GET::Invites{ invitetoken } => format!("/1/invites/{}", invitetoken.path()),
            GET::Users => String::from("/1/users/me"),
        }
    }
}

