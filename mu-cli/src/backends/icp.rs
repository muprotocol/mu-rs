use crate::{
    backends::render_template,
    project::{config::MuFunctionConfig, MuFunction},
    util::print_full_line,
};
use candid::TypeEnv;
use candid_parser::{typing, IDLProg};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    fs,
    net::TcpStream,
    path::Path,
    process::{Child, Command, Stdio},
    sync::{LazyLock, Mutex},
    thread::sleep,
    time::Duration,
};

static DFX_CONFIG_FILENAME: &str = "dfx.json";

static DFX_PROCESS: LazyLock<Mutex<Option<Child>>> = LazyLock::new(|| Mutex::new(None));

pub struct IcpFunction<'a> {
    function: &'a mut MuFunction,
    root: String,
}

impl<'a> IcpFunction<'a> {
    pub fn new(root: &str, function: &'a mut MuFunction) -> Self {
        let out = Self {
            function,
            root: root.to_string(),
        };
        out
    }

    pub fn save_dfx_config(&self) {
        IcpConfig::from(&self.function.config)
            .save(&format!("{}/{}", self.root, DFX_CONFIG_FILENAME));
    }

    pub fn init(&self) {
        let data = json!({
            "Cargo.toml": {
                "name":  self.function.config.name,
            }
        });

        render_template("icp/function", &self.root, data);
        self.save_dfx_config();
    }

    fn get_did_js(&self) -> Option<String> {
        let prog = self.function.state.unwrap_icp().did.as_ref()?;
        let ast = prog.parse::<IDLProg>().ok()?;
        let mut env = TypeEnv::new();
        let actor = typing::check_prog(&mut env, &ast).ok()?;
        Some(candid_parser::bindings::javascript::compile(&env, &actor))
    }

    pub fn build(&mut self) {
        print_full_line("Building ICP project");
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

        print_full_line("Extracting candid file");
        let candid = Command::new("candid-extractor")
            .arg(format!(
                "target/wasm32-unknown-unknown/release/{}.wasm",
                self.function.config.name
            ))
            .current_dir(&self.root)
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to extract candid file");

        if candid.status.code().unwrap() != 0 {
            eprintln!("Failed to extract candid file");
            std::process::exit(1);
        }

        // Store DID in the state and in a file
        self.function.state.unwrap_icp_mut().did =
            Some(String::from_utf8_lossy(&candid.stdout).into());

        let did_path = format!("{}/{}.did", self.root, self.function.config.name);
        fs::write(&did_path, &candid.stdout).unwrap();

        print_full_line("Generating JavaScript bindings");
        let js = self.get_did_js();

        if js.is_none() {
            eprintln!("Failed to generate JavaScript bindings");
            std::process::exit(1);
        }
        self.function.state.unwrap_icp_mut().js_bindings = js;
    }

    pub fn deploy(&mut self) {
        Self::start();

        print_full_line("Deploying ICP project...");
        let r = Command::new("dfx")
            .arg("deploy")
            .current_dir(&self.root)
            .status()
            .expect("Failed to deploy ICP project");

        if !r.success() {
            eprintln!("Failed to deploy ICP project");
            std::process::exit(1);
        }

        let canister_ids_json =
            fs::read_to_string(format!("{}/.dfx/local/canister_ids.json", self.root)).unwrap();
        let canister_ids: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(&canister_ids_json).unwrap();

        let canister_id = canister_ids
            .get(self.function.config.name.as_str())
            .unwrap()
            .get("local")
            .unwrap();
        self.function.state.unwrap_icp_mut().canister_id = Some(canister_id.to_string());
    }

    fn start() {
        if DFX_PROCESS.lock().unwrap().is_some() {
            return;
        }

        print_full_line("Starting local ICP node...");
        let dfx = Command::new("dfx")
            .arg("start")
            .current_dir(".")
            .spawn()
            .expect("Failed to start dfx");

        *DFX_PROCESS.lock().unwrap() = Some(dfx);

        loop {
            let conn = TcpStream::connect("localhost:4943");
            if conn.is_ok() {
                break;
            }
            sleep(Duration::from_secs(1));
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IcpConfig {
    canisters: HashMap<String, Canister>,
    defaults: Defaults,
    output_env_file: String,
    version: u32,
}

impl From<&MuFunctionConfig> for IcpConfig {
    fn from(config: &MuFunctionConfig) -> Self {
        let mut canisters = HashMap::new();
        canisters.insert(
            config.name.clone(),
            Canister {
                candid: format!("{}.did", config.name),
                package: config.name.clone(),
                canister_type: "rust".to_string(),
            },
        );

        Self {
            canisters,
            defaults: Default::default(),
            output_env_file: ".env".to_string(),
            version: 1,
        }
    }
}

impl IcpConfig {
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

#[derive(Serialize, Deserialize, Debug, Default)]
struct Defaults {
    build: Build,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Build {
    args: String,
    packtool: String,
}
