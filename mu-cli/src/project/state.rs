use serde::{Deserialize, Serialize};

use super::MuFunctionType;

#[derive(Serialize, Deserialize, Debug)]
pub struct MuProjectState {
    pub functions: Vec<MuFunctionState>,
}

impl MuProjectState {
    pub fn save(&self) {
        let json = serde_json::to_string(&self).unwrap();
        std::fs::write("mu.state.json", json).unwrap();
    }

    pub fn load() -> Option<MuProjectState> {
        if let Ok(json) = std::fs::read_to_string("mu.state.json") {
            let state = serde_json::from_str(&json).unwrap();
            Some(state)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MuFunctionState {
    pub name: String,
    pub backend_state: MuBackendFunctionState,
}

impl MuFunctionState {
    pub fn new(name: &str, fn_type: MuFunctionType) -> MuFunctionState {
        MuFunctionState {
            name: name.to_owned(),
            backend_state: match fn_type {
                MuFunctionType::ICP => MuBackendFunctionState::Icp(Default::default()),
                MuFunctionType::Solana => MuBackendFunctionState::Solana(Default::default()),
            },
        }
    }

    pub fn unwrap_icp(&self) -> &MuIcpFunctionState {
        match &self.backend_state {
            MuBackendFunctionState::Icp(icp) => icp,
            _ => panic!("Expected ICP backend state"),
        }
    }

    pub fn unwrap_icp_mut(&mut self) -> &mut MuIcpFunctionState {
        match &mut self.backend_state {
            MuBackendFunctionState::Icp(icp) => icp,
            _ => panic!("Expected ICP backend state"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MuBackendFunctionState {
    #[serde(rename = "icp")]
    Icp(MuIcpFunctionState),
    #[serde(rename = "solana")]
    Solana(MuSolanaFunctionState),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MuIcpFunctionState {
    pub did: Option<String>,
    pub canister_id: Option<String>,
    pub js_bindings: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MuSolanaFunctionState {
    pub name: String,
}
