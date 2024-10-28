use std::{fs, path::Path};

use rust_embed::Embed;
use serde_json::Value;

pub mod icp;

#[derive(Embed)]
#[folder = "src/backends/templates"]
struct Templates;

pub trait FunctionBackend {
    fn new_function(&mut self, name: String);

    fn build(&self);
}

pub fn render_template(template_name: &str, destination: &str, data: Value) {
    let handlebars = handlebars::Handlebars::new();
    let templates = Templates::iter()
        .map(|x| Path::new(&*x).to_owned())
        .filter(|x| x.starts_with(template_name));

    for path in templates {
        let relative_path = path.strip_prefix(&template_name).unwrap().to_str().unwrap();
        let template_data = data.get(relative_path);

        // Apply the template to the destination path
        let destination_path = format!("{}/{}", destination, relative_path);
        let destination_path = handlebars
            .render_template(&destination_path, &template_data)
            .unwrap();

        // Render file
        let template_raw = Templates::get(path.to_str().unwrap()).unwrap().data;
        let template = String::from_utf8_lossy(&template_raw);

        let contents = match template_data {
            Some(data) => handlebars.render_template(&template, data).unwrap(),
            None => template.to_string(),
        };

        if let Some(parent) = Path::new(&destination_path).parent() {
            fs::create_dir_all(parent).unwrap();
        }

        fs::write(destination_path, contents).unwrap();
    }
}
