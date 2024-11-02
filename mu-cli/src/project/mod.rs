use clap::ValueEnum;
use config::{MuFrontendConfig, MuFunctionConfig, MuProjectConfig, MuProjectMetadata};
use serde::{Deserialize, Serialize};
use state::{MuBackendFunctionState, MuFunctionState, MuProjectState};

use crate::{
    backends::{icp::IcpFunction, js::JsBackend},
    util::print_full_line,
};

pub mod config;
pub mod state;

pub struct MuProject {
    pub metadata: MuProjectMetadata,
    pub functions: Vec<MuFunction>,
    pub frontends: Vec<MuFrontend>,
}

impl MuProject {
    pub fn init(name: String) {
        let metadata = MuProjectMetadata {
            name,
            version: "0.1.0".to_string(),
            description: "A new Mu project".to_string(),
        };

        print_full_line(&format!("Initializing project: {}", metadata.name));
        let project = MuProject {
            metadata,
            functions: vec![],
            frontends: vec![],
        };

        project.save();

        print_full_line("Project initialized.");
    }

    pub fn save(&self) {
        let config = self.as_config();
        config.save();

        let state = self.as_state();
        state.save();
    }

    pub fn load() -> Option<MuProject> {
        let config = MuProjectConfig::load()?;
        let state = MuProjectState::load()?;

        let functions = config
            .functions
            .into_iter()
            .zip(state.functions.into_iter())
            .map(|(config, state)| MuFunction { config, state })
            .collect();

        Some(MuProject {
            metadata: config.metadata,
            functions,
            frontends: vec![],
        })
    }

    pub fn as_config(&self) -> MuProjectConfig {
        MuProjectConfig {
            frontends: self.frontends.iter().map(|f| f.config.clone()).collect(),
            functions: self.functions.iter().map(|f| f.config.clone()).collect(),
            metadata: self.metadata.clone(),
        }
    }

    pub fn as_state(&self) -> MuProjectState {
        MuProjectState {
            functions: self.functions.iter().map(|f| f.state.clone()).collect(),
        }
    }

    pub fn add_function(&mut self, name: &str, fn_type: MuFunctionType) {
        print_full_line(&format!("Adding function: {}", name));

        let path = format!("functions/{}", name);
        std::fs::create_dir_all(path).unwrap();

        let mut function = MuFunction::new(name, fn_type);
        function.init();
        self.functions.push(function);

        self.save();
    }

    pub fn dev(&mut self) {
        self.frontends.iter().for_each(|f| f.dev());
        self.functions.iter_mut().for_each(|f| f.dev());
    }

    pub fn build(&mut self) {
        self.functions.iter_mut().for_each(|f| f.build());
        self.save();
    }

    pub fn deploy(&mut self) {
        self.functions.iter_mut().for_each(|f| f.deploy());
        self.save();
    }

    pub fn add_frontend(&mut self, name: &str, template: MuFrontendTemplate) {
        print_full_line(&format!("Adding frontend: {}", name));

        // create the diretory frontends/name
        let path = format!("frontends/{}", name);
        std::fs::create_dir_all(path).unwrap();

        let fe = MuFrontend::new(name, template);

        self.frontends.push(fe);
        self.save();
    }
}

pub struct MuFunction {
    pub state: MuFunctionState,
    pub config: MuFunctionConfig,
}

impl MuFunction {
    pub fn new(name: &str, fn_type: MuFunctionType) -> MuFunction {
        MuFunction {
            state: MuFunctionState::new(name, fn_type),
            config: MuFunctionConfig::new(name, fn_type),
        }
    }

    // fn get_backend(&self) -> Box<dyn MuFunctionBackend> {
    //     match &self.state.backend_state {
    //         MuBackendFunctionState::Icp(_) => {
    //             IcpFunction::new(&format!("functions/{}", self.state.name), self)
    //         }
    //         MuBackendFunctionState::Solana(_) => unimplemented!(),
    //     }
    // }

    pub fn init(&mut self) {
        match &mut self.state.backend_state {
            MuBackendFunctionState::Icp(_icp) => {
                let icp_function =
                    IcpFunction::new(&format!("functions/{}", self.state.name), self);
                icp_function.init();
            }
            MuBackendFunctionState::Solana(_) => {
                unimplemented!();
            }
        }
    }

    pub fn build(&mut self) {
        match &self.state.backend_state {
            MuBackendFunctionState::Icp(_icp) => {
                let mut icp_function =
                    IcpFunction::new(&format!("functions/{}", self.state.name), self);
                icp_function.build();
            }
            MuBackendFunctionState::Solana(_) => {
                unimplemented!();
            }
        }
    }

    pub fn deploy(&mut self) {
        match &self.state.backend_state {
            MuBackendFunctionState::Icp(_icp) => {
                let mut icp_function =
                    IcpFunction::new(&format!("functions/{}", self.state.name), self);
                icp_function.deploy();
            }
            MuBackendFunctionState::Solana(_) => {
                unimplemented!();
            }
        }
    }

    pub fn dev(&mut self) {
        match &self.state.backend_state {
            MuBackendFunctionState::Icp(_icp) => {
                let mut icp_function =
                    IcpFunction::new(&format!("functions/{}", self.state.name), self);
                icp_function.dev();
            }
            MuBackendFunctionState::Solana(_) => {
                unimplemented!();
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MuFrontend {
    pub config: MuFrontendConfig,
}

impl MuFrontend {
    pub fn new(name: &str, template: MuFrontendTemplate) -> MuFrontend {
        let out = MuFrontend {
            config: MuFrontendConfig {
                name: name.to_owned(),
                template,
            },
        };

        out.get_backend().create_frontend();
        out
    }

    fn get_backend(&self) -> JsBackend {
        JsBackend::new(&format!("frontends/{}", self.config.name), &self.config)
    }

    pub fn dev(&self) {
        self.get_backend().dev();
    }
}

#[derive(ValueEnum, Serialize, Deserialize, Debug, Clone)]
pub enum MuFrontendTemplate {
    Vanilla,
    React,
    Vue,
}

#[derive(ValueEnum, Deserialize, Serialize, Clone, Copy, Debug)]
pub enum MuFunctionType {
    ICP,
    Solana,
}

impl ToString for MuFunctionType {
    fn to_string(&self) -> String {
        match self {
            MuFunctionType::ICP => "icp".to_string(),
            MuFunctionType::Solana => "solana".to_string(),
        }
    }
}
