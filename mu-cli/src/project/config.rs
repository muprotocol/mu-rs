use serde::{Deserialize, Serialize};

use super::{MuFrontendTemplate, MuFunctionType};

#[derive(Serialize, Deserialize, Debug)]
pub struct MuProjectConfig {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub functions: Vec<MuFunctionConfig>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub frontends: Vec<MuFrontendConfig>,

    pub metadata: MuProjectMetadata,
}

impl MuProjectConfig {
    pub fn load() -> Option<MuProjectConfig> {
        if let Ok(toml) = std::fs::read_to_string("mu.toml") {
            let config = toml::from_str::<MuProjectConfig>(&toml).unwrap();
            Some(config)
        } else {
            None
        }
    }

    pub fn save(&self) {
        let toml = toml::to_string(&self).unwrap();
        std::fs::write("mu.toml", toml).unwrap();
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename = "project")]
pub struct MuProjectMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MuFunctionConfig {
    pub name: String,
    pub fn_type: MuFunctionType,
}

impl MuFunctionConfig {
    pub fn new(name: &str, fn_type: MuFunctionType) -> MuFunctionConfig {
        MuFunctionConfig {
            name: name.to_owned(),
            fn_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MuFrontendConfig {
    pub name: String,
    pub template: MuFrontendTemplate,
}
