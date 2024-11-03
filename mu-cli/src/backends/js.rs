use tokio::{
    net::TcpStream,
    process::Command,
    sync::mpsc::{self, UnboundedReceiver},
    time::sleep,
    time::Duration,
};

use serde_json::json;

use super::render_template;
use crate::{
    project::{config::MuFrontendConfig, MuFrontendTemplate},
    util::print_full_line,
};

pub struct JsBackend<'a> {
    config: &'a MuFrontendConfig,
    root: String,
}

impl<'a> JsBackend<'a> {
    pub fn new(root: &str, config: &'a MuFrontendConfig) -> JsBackend<'a> {
        JsBackend {
            config,
            root: root.to_owned(),
        }
    }

    pub fn create_frontend(&self) {
        let data: serde_json::Value = json!({
            "package.json": {
                "name": self.config.name,
            }
        });

        let template_path = match self.config.template {
            MuFrontendTemplate::Vanilla => "js/vanilla",
            MuFrontendTemplate::Vue => unimplemented!(),
            MuFrontendTemplate::React => unimplemented!(),
        };

        render_template(template_path, &self.root, data);

        std::process::Command::new("npm")
            .arg("install")
            .current_dir(&self.root)
            .status()
            .expect("Failed to install dependencies");

        print_full_line("Frontend created!");
    }

    pub fn dev(self) -> UnboundedReceiver<()> {
        let (tx, rx) = mpsc::unbounded_channel();
        print_full_line("Dev server started at http://localhost:5173");
        tokio::spawn(async move {
            let mut child = Command::new("npm")
                .arg("run")
                .arg("dev")
                .current_dir(&self.root)
                .spawn()
                .expect("Failed to start dev server");

            loop {
                let conn = TcpStream::connect("localhost:5173").await;
                if conn.is_ok() {
                    break;
                }
                sleep(Duration::from_secs(1)).await;
            }

            tx.send(()).unwrap();

            child.wait().await.unwrap();
        });
        rx
    }
}
