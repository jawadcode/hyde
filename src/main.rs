use std::{io, path::PathBuf};

use clap::{Parser, Subcommand};
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
        /// The display name of the site
        name: String,
        /// A description of the site
        #[arg(short, long)]
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
    #[snafu(display("Failed to create project '{}' at '{}': {}", name, path.display(), source))]
    New {
        source: io::Error,
        name: String,
        path: PathBuf,
    },

    #[snafu(display("Failed to build project at '{}': {}", path.display(), source))]
    Build { source: io::Error, path: PathBuf },

    #[snafu(display("Failed to serve project at '{}': {}", path.display(), source))]
    Serve { source: io::Error, path: PathBuf },
}

fn main() {
    if let Err(err) = run() {
        eprintln!("\x1b[31;1mError\x1b[0m: {err}");
    }
}

type AppRes = std::result::Result<(), AppError>;

fn run() -> AppRes {
    match Args::parse().command {
        Command::New { name: _, desc: _ } => todo!(),
        Command::Build => todo!(),
        Command::Serve => todo!(),
    }
}
