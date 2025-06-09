mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use commands::*;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Historia => historia::run(),
        Commands::Caracteristicas => caracteristicas::run(),
        Commands::Paradigmas => paradigmas::run(),
        Commands::Exemplo { tema } => exemplo::run(tema),
        Commands::Tutorial => tutorial::run(),
        Commands::Sobre => sobre::run(),
        Commands::Mascote => mascote::run(),
        Commands::ExecuteScript {lang, file} => execute_script::run(lang, file),
    }
}
