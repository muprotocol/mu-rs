use clap::{Parser, Subcommand};
use project::{MuFrontendTemplate, MuFunctionType, MuProject};
use util::print_full_line;

mod backends;
mod project;
pub mod util;

#[derive(Parser)]
#[command(version, about)]
struct MuCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project
    Init { name: Option<String> },

    /// Work with Mu functions
    Function {
        #[command(subcommand)]
        command: Function,
    },

    /// Work with Mu frontends
    Frontend {
        #[command(subcommand)]
        command: Frontend,
    },

    /// Build the project
    Build,

    /// Deploy the project
    Deploy,

    /// Run the project in development mode
    Dev,
}

#[derive(Subcommand)]
enum Function {
    /// Adds a new function
    Add {
        name: String,
        #[arg(id = "TYPE")]
        fn_type: MuFunctionType,
    },
}

#[derive(Subcommand)]
enum Frontend {
    /// Adds a new frontend
    Add {
        name: String,
        #[arg(id = "TYPE")]
        template: MuFrontendTemplate,
    },
}

fn get_project() -> MuProject {
    let project = MuProject::load();
    if project.is_none() {
        eprintln!("No project found. Run `mu init` first.");
        std::process::exit(1);
    }
    project.unwrap()
}

fn main() {
    let cli = MuCli::parse();

    print_full_line("Welcome to Mu [μ]!");

    match cli.command {
        Commands::Function { command } => match command {
            Function::Add { name, fn_type } => {
                get_project().add_function(&name, fn_type);
            }
        },
        Commands::Init { name } => {
            let name = if let Some(name) = name {
                name
            } else {
                std::env::current_dir()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            };

            MuProject::init(name);
        }
        Commands::Frontend { command } => match command {
            Frontend::Add { name, template } => {
                get_project().add_frontend(&name, template);
            }
        },
        Commands::Build => {
            get_project().build();
        }
        Commands::Deploy => {
            get_project().deploy();
        }
        Commands::Dev => {
            get_project().dev();
        }
    }
}
