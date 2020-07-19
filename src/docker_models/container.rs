use super::port::Port;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
// use std::fmt::Display;

use super::Du;
// use super::Di;

#[allow(non_snake_case)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Container {
    pub Id: String,
    pub Names: Vec<String>,
    pub Image: String,
    pub ImageID: String,
    pub Command: String,
    pub Created: Du,
    pub State: String,
    pub Status: String,
    pub Ports: Option<Vec<Port>>,
    pub Labels: Option<BTreeMap<String, String>>,
    pub SizeRw: Du,
    pub SizeRootFs: Du,
    pub HostConfig: Option<BTreeMap<String, String>>,
    pub NetworkSettings: Option<BTreeMap<String, BTreeMap<String, String>>>,
    pub Mounts: Option<Vec<BTreeMap<String, String>>>,
}
