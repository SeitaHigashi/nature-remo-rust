pub mod appliance;
pub mod device;
pub mod home;
pub mod signal;

use super::api::API;
use appliance::Appliance;
use device::Device;
use home::Home;
use signal::Signal;

pub enum POST {
    ApplianceOrders,
    Appliances{applianceid: Option<String>, appliance: Option<Appliance>},
    DetectAppliance,
    Devices{deviceid: String, device: Option<Device>},
    Homes{homeid: Option<String>, home: Option<Home>},
    Invites{invitetoken: String},
    Signals{signalid: String, signal: Option<Signal>},
    Users,
}

impl API for POST {
    fn path(&self) -> String {
        match self {
            POST::ApplianceOrders => String::from("/1/appliance_orders"),
            POST::Appliances { applianceid, appliance } => String::from("/1/appliances"),
            POST::DetectAppliance => String::from("/1/detectappliance"),
            POST::Devices { deviceid, device } => String::from("/1/devices"),
            POST::Homes { homeid, home } => String::from("/1/homes"),
            POST::Invites { invitetoken } => String::from("/1/invites"),
            POST::Signals { signalid, signal } => String::from("/1/signals"),
            POST::Users => String::from("/1/users"),
        }
    }
}
