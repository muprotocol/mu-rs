use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    fs,
    path::Path,
    process::{Command, Stdio},
};

use crate::backends::render_template;

use super::FunctionBackend;

static CONFIG_FILENAME: &str = "dfx.json";

pub struct IcpBackend {
    config: IcpConfig,
    root: String,
}

impl IcpBackend {
    pub fn new(root: &str) -> Self {
        let config = IcpConfig::load_or_default(&format!("{}/{}", root, CONFIG_FILENAME));
        Self {
            config,
            root: root.to_string(),
        }
    }

    pub fn save_config(&self) {
        self.config
            .save(&format!("{}/{}", self.root, CONFIG_FILENAME));
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct IcpConfig {
    canisters: HashMap<String, Canister>,
    defaults: Defaults,
    output_env_file: String,
    version: u32,
}

impl IcpConfig {
    fn load_or_default(path: &str) -> Self {
        if Path::new(path).exists() {
            let json_data = fs::read_to_string(path);
            match json_data {
                Ok(data) => {
                    if let Ok(config) = serde_json::from_str(&data) {
                        return config;
                    }
                }
                Err(e) => eprintln!("Failed to read config file: {}", e),
            }
        }

        // Default configuration if loading fails
        Self {
            canisters: HashMap::new(),
            defaults: Defaults {
                build: Build {
                    args: "".to_string(),
                    packtool: "".to_string(),
                },
            },
            output_env_file: ".env".to_string(),
            version: 1,
        }
    }

    fn save(&self, path: &str) {
        let serialized = serde_json::to_string_pretty(&self).unwrap();
        fs::write(path, serialized).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Canister {
    candid: String,
    package: String,
    #[serde(rename = "type")]
    canister_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Defaults {
    build: Build,
}

#[derive(Serialize, Deserialize, Debug)]
struct Build {
    args: String,
    packtool: String,
}

impl FunctionBackend for IcpBackend {
    fn new_function(&mut self, name: String) {
        self.config.canisters.insert(
            name.clone(),
            Canister {
                candid: format!("{}.did", name),
                package: name.clone(),
                canister_type: "rust".to_string(),
            },
        );
        self.save_config();

        let data = json!({
            "Cargo.toml": {
                "name": name,
            }
        });

        render_template("icp/function", &self.root, data);

        println!("Adding ICP function: {}", name);
    }

    fn build(&self) {
        println!("Building ICP project");
        let r = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .arg("--target")
            .arg("wasm32-unknown-unknown")
            .current_dir(&self.root)
            .status()
            .expect("Failed to build ICP project");

        if !r.success() {
            eprintln!("Failed to build ICP project");
            std::process::exit(1);
        }

        println!("Extracting candid file");
        let candid = Command::new("candid-extractor")
            .arg(format!(
                "target/wasm32-unknown-unknown/release/{}.wasm",
                self.config.canisters.keys().next().unwrap()
            ))
            .current_dir(&self.root)
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to extract candid file");

        if candid.status.code().unwrap() != 0 {
            eprintln!("Failed to extract candid file");
            std::process::exit(1);
        }

        let candid_path = format!(
            "{}/{}.did",
            self.root,
            self.config.canisters.keys().next().unwrap()
        );
        fs::write(&candid_path, candid.stdout).unwrap();

        println!("Generating JavaScript bindings");

        let js = Command::new("didc")
            .arg("bind")
            .arg("--target")
            .arg("js")
            .arg(format!(
                "{}.did",
                self.config.canisters.keys().next().unwrap()
            ))
            .current_dir(&self.root)
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to generate JavaScript bindings");
        if js.status.code().unwrap() != 0 {
            eprintln!("Failed to generate JavaScript bindings");
            std::process::exit(1);
        }

        let js_path = format!(
            "{}/{}.js",
            self.root,
            self.config.canisters.keys().next().unwrap()
        );
        fs::write(&js_path, js.stdout).unwrap();
    }
}
