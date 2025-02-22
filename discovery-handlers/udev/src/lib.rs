extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate udev;
#[macro_use]
extern crate serde_derive;

pub mod discovery_handler;
mod discovery_impl;
mod wrappers;

/// Name of environment variable that is set in udev brokers. Contains devnode for udev device
/// the broker should connect to.
pub const UDEV_DEVNODE_LABEL_ID: &str = "UDEV_DEVNODE";
/// Name that udev discovery handlers use when registering with the Agent
pub const DISCOVERY_HANDLER_NAME: &str = "udev";
/// Defines whether this discovery handler discovers local devices on nodes rather than ones visible to multiple nodes
pub const SHARED: bool = false;

#[cfg(fuzzing)]
pub fn parse_udev_rule(rule: &str) {
    _ = discovery_impl::parse_udev_rule(rule);
}
