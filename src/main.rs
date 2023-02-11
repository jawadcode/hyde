use anyhow::Context;
use clap::{Parser, Subcommand};
use hyde::{build, new, serve};
use std::{env, path::PathBuf, process};

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

fn main() {
    if let Err(err) = run() {
        eprintln!("\x1b[31;1mError\x1b[0m: {err}");
        err.chain()
            .skip(1)
            .for_each(|cause| eprintln!("    \x1b[37mCaused By\x1b[0m: {cause}"));
        process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    match Args::parse().command {
        Command::New { name, desc } => new::new(&name, desc.as_deref(), cwd()?)
            .with_context(|| format!("Failed to create project '{name}'"))?,
        Command::Build => build::build(cwd()?).context("Failed to build project")?,
        Command::Serve => serve::serve(cwd()?).context("Failed to serve project")?,
    };

    Ok(())
}

fn cwd() -> anyhow::Result<PathBuf> {
    env::current_dir().context("Failed to read current directory")
}
