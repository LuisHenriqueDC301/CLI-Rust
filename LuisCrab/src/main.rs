use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustaid")]
#[command(about = "CLI interativa para aprender Rust 🦀", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Historia => {
            println!("🦀 Rust foi criado por Graydon Hoare em 2006 e lançado pela Mozilla em 2010...");
        }
        Commands::Caracteristicas => {
            println!("✅ Segurança de memória sem garbage collector\n✅ Concorrência eficiente\n✅ Zero-cost abstractions...");
        }
        Commands::Paradigmas => {
            println!("💡 Rust suporta paradigmas imperativo, funcional, orientado a objetos e concorrente.");
        }
        Commands::Exemplo { tema } => {
            match tema.as_str() {
                "ownership" => {
                   println!("🔧 Exemplo de Ownership:\n\nfn main() {{\n    let s = String::from(\"hello\");\n    let t = s;\n    println!(\"{{}}\", t);\n}}");

                }
                "borrowing" => {
                    println!("🔧 Exemplo de Borrowing:\n\nfn main() {{\n    let s = String::from(\"hello\");\n    print_str(&s);\n}}\n\nfn print_str(s: &String) {{\n    println!(\"{{}}\", s);\n}}");
                }
                _ => println!("❌ Tema não reconhecido."),
            }
        }
        Commands::Tutorial => {
            println!("📘 Instale o Rust com: https://rustup.rs\nUse 'cargo new projeto' para criar projetos.");
        }
        Commands::Sobre => {
            println!("Este projeto foi criado como parte de um seminário da PUC Minas.\nAutor: Seu Nome Aqui");
        }
    }
}
