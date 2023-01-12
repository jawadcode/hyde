use anyhow::Context;
use clap::{Parser, Subcommand};
use hyde::{build, new, serve};
use std::{env, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple SSG for creating blogs", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a new hyde project
    New {
        name: String,
        #[arg(short, long)]
        desc: Option<String>,
    },
    /// Build the project in the current directory
    Build,
    /// Serve the project in the current directory
    Serve,
}

fn main() -> anyhow::Result<()> {
    match Args::parse().command {
        Command::New { name, desc } => new::new(&name, desc.as_deref(), cwd()?)
            .with_context(|| format!("Couldn't create project '{name}'"))?,
        Command::Build => build::build(cwd()?).with_context(|| "Couldn't build project")?,
        Command::Serve => serve::serve(cwd()?).with_context(|| "Couldn't serve project")?,
    };

    Ok(())
}

fn cwd() -> anyhow::Result<PathBuf> {
    env::current_dir().with_context(|| "Couldn't read current directory")
}
