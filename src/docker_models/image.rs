use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
// use std::fmt::Display;

use super::Di;
use super::Du;

#[allow(non_snake_case)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Image {
    pub Id: String,
    pub ParentId: Option<String>,
    pub RepoTags: Option<Vec<String>>,
    pub RepoDigests: Option<Vec<String>>,
    pub Created: Du,
    pub Size: Du,
    pub VirtualSize: Du,
    pub SharedSize: Di,
    pub Labels: Option<BTreeMap<String, String>>,
    pub Containers: Di,
}
