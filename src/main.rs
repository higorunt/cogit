use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod cogit;

use cogit::CogitRepository;

#[derive(Parser)]
#[command(name = "cogit")]
#[command(about = "COGIT - Cognition Git: Um sistema de controle de versão em Rust")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inicializa um novo repositório COGIT
    Init {
        /// Diretório para inicializar (padrão: diretório atual)
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Cria um novo commit com as mudanças atuais
    Commit {
        /// Mensagem do commit
        #[arg(short, long)]
        message: String,
    },
    /// Mostra o histórico de commits
    Log,
    /// Mostra o status atual do repositório
    Status,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            match CogitRepository::init(&path) {
                Ok(_) => println!("✅ Repositório COGIT inicializado em: {}", path.display()),
                Err(e) => eprintln!("❌ Erro ao inicializar repositório: {}", e),
            }
        }
        Commands::Commit { message } => {
            match CogitRepository::open(".") {
                Ok(mut repo) => {
                    match repo.commit(&message) {
                        Ok(hash) => println!("✅ Commit criado: {}", hash),
                        Err(e) => eprintln!("❌ Erro ao criar commit: {}", e),
                    }
                }
                Err(e) => eprintln!("❌ Erro: {}", e),
            }
        }
        Commands::Log => {
            match CogitRepository::open(".") {
                Ok(repo) => {
                    match repo.log() {
                        Ok(commits) => {
                            if commits.is_empty() {
                                println!("📝 Nenhum commit encontrado");
                            } else {
                                for commit in commits {
                                    println!("🔹 {} - {}", commit.hash, commit.message);
                                    println!("   📅 {}", commit.timestamp);
                                    println!();
                                }
                            }
                        }
                        Err(e) => eprintln!("❌ Erro ao buscar histórico: {}", e),
                    }
                }
                Err(e) => eprintln!("❌ Erro: {}", e),
            }
        }
        Commands::Status => {
            match CogitRepository::open(".") {
                Ok(repo) => {
                    match repo.status() {
                        Ok(status) => println!("📊 Status: {}", status),
                        Err(e) => eprintln!("❌ Erro ao verificar status: {}", e),
                    }
                }
                Err(e) => eprintln!("❌ Erro: {}", e),
            }
        }
    }
}
