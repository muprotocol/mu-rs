use clap::{Parser, Subcommand};
use project::{FunctionType, MuProject};

mod backends;
mod project;

#[derive(Parser)]
#[command(version, about)]
struct MuCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project
    Init {
        name: Option<String>,
    },

    /// Work with Mu functions
    Function {
        #[command(subcommand)]
        command: Function,
    },

    Build,
}

#[derive(Subcommand)]
enum Function {
    /// Adds a new function
    Add {
        name: String,
        #[arg(id = "TYPE")]
        fn_type: FunctionType,
    },
}

fn main() {
    let cli = MuCli::parse();

    match cli.command {
        Commands::Function { command } => match command {
            Function::Add { name, fn_type } => {
                let project = MuProject::load();
                if project.is_none() {
                    eprintln!("No project found. Run `mu init` first.");
                    std::process::exit(1);
                }

                project.unwrap().add_function(&name, fn_type);
            }
        },
        Commands::Init { name } => {
            let name = if let Some(name) = name {
                name
            } else {
                // current directory name
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
        Commands::Build => {
            let project = MuProject::load();
            if project.is_none() {
                eprintln!("No project found. Run `mu init` first.");
                std::process::exit(1);
            }

            let project = project.unwrap();
            project.build();
        }
    }
}
