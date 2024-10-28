use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use crate::backends::{icp::IcpBackend, FunctionBackend};

#[derive(Deserialize, Serialize)]
pub struct MuProject {
    pub project: MuProjectMetadata,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub functions: Vec<MuFunction>,
}

impl MuProject {
    pub fn load() -> Option<MuProject> {
        if let Ok(toml) = std::fs::read_to_string("mu.toml") {
            let config: MuProject = toml::from_str(&toml).unwrap();
            Some(config)
        } else {
            None
        }
    }

    pub fn save(&self) {
        let toml = toml::to_string(&self).unwrap();
        std::fs::write("mu.toml", toml).unwrap();
    }

    pub fn init(name: String) {
        let project = MuProjectMetadata {
            name,
            version: "0.1.0".to_string(),
            description: "A new Mu project".to_string(),
        };

        println!("Initializing project: {}", project.name);
        let config = MuProject {
            project,
            functions: vec![],
        };

        config.save();

        println!("Project initialized.");
    }

    pub fn add_function(&mut self, name: &str, fn_type: FunctionType) {
        println!("Adding function: {}", name);

        // create the diretory functions/type/name
        let path = format!("functions/{}", name);
        std::fs::create_dir_all(path).unwrap();

        match fn_type {
            FunctionType::ICP => {
                self.functions.push(MuFunction {
                    name: name.to_owned(),
                    fn_type,
                });

                let mut icp_backend = IcpBackend::new(&format!("functions/{}", name));
                icp_backend.new_function(name.to_owned());
            }
            FunctionType::Solana => {
                unimplemented!();
            }
        }

        self.save();
    }

    pub fn build(&self) {
        for function in &self.functions {
            match function.fn_type {
                FunctionType::ICP => {
                    let icp_backend = IcpBackend::new(&format!("functions/{}", function.name));
                    icp_backend.build();
                }
                FunctionType::Solana => {
                    unimplemented!();
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "project")]
pub struct MuProjectMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct MuFunction {
    pub name: String,
    pub fn_type: FunctionType,
}

#[derive(ValueEnum, Deserialize, Serialize, Clone, Copy, Debug)]
pub enum FunctionType {
    ICP,
    Solana,
}

impl ToString for FunctionType {
    fn to_string(&self) -> String {
        match self {
            FunctionType::ICP => "icp".to_string(),
            FunctionType::Solana => "solana".to_string(),
        }
    }
}
