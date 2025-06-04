use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "Luiscrab")]
#[command(about = "CLI interativa para aprender Rust 🦀", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
   pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Mostra a história da linguagem Rust
    Historia,

    /// Mostra as principais características da linguagem Rust
    Caracteristicas,

    /// Mostra os paradigmas da linguagem
    Paradigmas,

    /// Executa um exemplo de código
    Exemplo {
        #[arg(short, long)]
        tema: String,
    },

    /// Mostra um pequeno tutorial de instalação e uso
    Tutorial,
    
    /// Sobre o projeto
    Sobre,

    //Mostra o Crab,
    Mascote,

    //Compara o tempo de execucao entre rust e outro script 
    CompareTime,
}