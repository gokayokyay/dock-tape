use serde::{Deserialize, Serialize};

type Du = u64;

#[allow(non_snake_case)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct Port {
    pub PrivatePort: Option<Du>,
    pub PublicPort: Option<Du>,
    pub Type: Option<String>,
    pub IP: Option<String>,
}
