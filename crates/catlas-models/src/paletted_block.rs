use std::{collections::HashMap, fmt::Display, hash::BuildHasherDefault};

use rustc_hash::FxHasher;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PalettedBlock {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Properties")]
    pub properties: Option<Properties>,
}

type Hasher = BuildHasherDefault<FxHasher>;
pub type Properties = HashMap<String, String, Hasher>;

impl Display for PalettedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(properties) = &self.properties {
            if properties.len() == 0 {
                return write!(f, "{}", self.name.clone());
            };
            let mut properties = properties.iter().collect::<Vec<_>>();

            properties.sort_by(|(key_a, _), (key_b, _)| key_a.cmp(key_b));

            let properties = properties
                .iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<_>>()
                .join(",");

            write!(f, "{}[{}]", self.name, properties)
        } else {
            write!(f, "{}", self.name.clone())
        }
    }
}
