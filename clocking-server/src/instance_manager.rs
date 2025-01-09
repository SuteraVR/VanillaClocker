use servify;

use crate::instance::{Instance, InstanceID, PlayerID};
use std::collections::HashMap;

#[derive(Debug)]
pub enum InstanceManagerMessage {
    NewInstance(InstanceID),
    JoinInstance(PlayerID),
}

#[servify::service(
    impls = [
        InstanceManager_new_instance,
    ]
)]
pub struct InstanceManager {
    instances: HashMap<InstanceID, PlayerID>,
}

#[servify::export]
impl InstanceManager {
    fn new_instance(&mut self, id: InstanceID) {
        let instance = Instance::new(id);
    }
}
