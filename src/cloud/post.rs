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
    AppliancesOrders,
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
        todo!()
    }
}
