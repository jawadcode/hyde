use std::{env, io, path::PathBuf};

use clap::{Parser, Subcommand};
use hyde::{
    build,
    new::{self, CreateError},
};
use snafu::Snafu;

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple SSG tailored towards blogs.", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a new Hyde project
    New {
        name: String,
        /// The display name of the site
        #[arg(long)]
        display_name: String,
        /// A description of the site
        #[arg(long)]
        desc: Option<String>,
    },
    /// Build the project in the current working directory
    Build,
    /// Build and serve the resulting statically generated site.
    Serve,
}

#[allow(unused)]
#[derive(Debug, Snafu)]
enum AppError {
    #[snafu(display("Failed to create project '{name}' at '{}': {source}", path.display()))]
    New {
        source: CreateError,
        name: String,
        path: PathBuf,
    },

    #[snafu(display("Failed to build project at '{}': {source}", path.display()))]
    Build {
        source: Box<build::BuildError>,
        path: PathBuf,
    },

    #[snafu(display("Failed to serve project at '{}': {source}", path.display()))]
    Serve { source: io::Error, path: PathBuf },

    #[snafu(display("Failed to get current directory: {source}"))]
    CurrentDir { source: io::Error },
}

fn main() {
    if let Err(err) = run() {
        eprintln!("\x1b[31;1mError\x1b[0m: {err}");
    }
}

type AppRes = std::result::Result<(), AppError>;

fn run() -> AppRes {
    let dir = env::current_dir().map_err(|source| AppError::CurrentDir { source })?;
    match Args::parse().command {
        Command::New {
            ref name,
            ref display_name,
            ref desc,
        } => new::new_project(&dir, name, display_name, desc.as_deref()).map_err(|source| {
            AppError::New {
                source,
                name: name.clone(),
                path: dir,
            }
        }),
        Command::Build => build::build_proj(&dir).map_err(|source| AppError::Build {
            source: Box::new(source),
            path: dir,
        }),
        Command::Serve => todo!(),
    }
}
