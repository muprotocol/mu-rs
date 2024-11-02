use std::process::Command;

use serde_json::json;

use super::render_template;
use crate::project::{config::MuFrontendConfig, MuFrontendTemplate};

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
    }

    pub fn dev(&self) {
        // serve with npm run dev
        Command::new("npm")
            .arg("run")
            .arg("dev")
            .current_dir(&self.root)
            .spawn()
            .expect("Failed to start dev server");
    }
}
