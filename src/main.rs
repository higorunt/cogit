use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod cogit;
mod embedding;

use cogit::CogitRepository;
use embedding::EmbeddingEngine;

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
        /// Pular análise de embeddings IA (modo rápido)
        #[arg(long, default_value = "false")]
        skip_ai: bool,
    },
    /// Mostra o histórico de commits
    Log,
    /// Mostra o status atual do repositório
    Status,
    /// Explica um commit usando IA (requer hash do commit)
    Explain {
        /// Hash do commit para explicar
        commit_hash: String,
    },
    /// Lista todos os commits com embeddings IA disponíveis
    Index,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            match CogitRepository::init(&path) {
                Ok(_) => println!("✅ Repositório COGIT inicializado em: {}", path.display()),
                Err(e) => eprintln!("❌ Erro ao inicializar repositório: {}", e),
            }
        }
        Commands::Commit { message, skip_ai } => {
            match CogitRepository::open(".") {
                Ok(mut repo) => {
                    match repo.commit(&message) {
                        Ok(hash) => {
                            println!("✅ Commit criado: {}", hash);
                            
                            // Processar embeddings IA se não foi pulado
                            if !skip_ai {
                                println!("🧠 Iniciando análise semântica...");
                                
                                // Obter chave da API via variável de ambiente
                                if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
                                    let cogit_dir = std::path::Path::new(".").join(".cogit");
                                    match EmbeddingEngine::new(cogit_dir) {
                                        Ok(mut engine) => {
                                            engine.set_api_key(api_key);
                                            
                                            match engine.process_commit_embeddings(&hash, std::path::Path::new(".")).await {
                                                Ok(index) => {
                                                    println!("🎯 Análise concluída: {} arquivo(s) processado(s)", index.files.len());
                                                    println!("⏱️  Tempo: {}ms | Tokens: {}", index.processing_time_ms, index.total_tokens);
                                                }
                                                Err(e) => {
                                                    eprintln!("⚠️  Erro na análise IA: {}", e);
                                                    println!("✅ Commit salvo sem embeddings");
                                                }
                                            }
                                        }
                                        Err(e) => eprintln!("⚠️  Erro ao inicializar motor IA: {}", e),
                                    }
                                } else {
                                    println!("💡 Para análise IA, defina: export OPENAI_API_KEY=sua_chave");
                                    println!("   Ou use --skip-ai para pular a análise");
                                }
                            }
                        }
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
                        Ok(status) => {
                            println!("📊 Status: {}", status);
                            
                            // Mostrar informações de IA se disponível
                            let cogit_dir = std::path::Path::new(".").join(".cogit");
                            if let Ok(engine) = EmbeddingEngine::new(cogit_dir) {
                                match engine.list_embedded_commits() {
                                    Ok(commits) => {
                                        if !commits.is_empty() {
                                            println!("🧠 Commits com análise IA: {}", commits.len());
                                        }
                                    }
                                    Err(_) => {} // Silenciar erros aqui
                                }
                            }
                        }
                        Err(e) => eprintln!("❌ Erro ao verificar status: {}", e),
                    }
                }
                Err(e) => eprintln!("❌ Erro: {}", e),
            }
        }
        Commands::Explain { commit_hash } => {
            let cogit_dir = std::path::Path::new(".").join(".cogit");
            match EmbeddingEngine::new(cogit_dir) {
                Ok(engine) => {
                    match engine.load_embedding_index(&commit_hash) {
                        Ok(index) => {
                            println!("🧠 Análise do Commit: {}", commit_hash);
                            println!("📅 Criado em: {}", index.created_at);
                            println!("📁 Arquivos analisados: {}", index.files.len());
                            println!("🎯 Tokens processados: {}", index.total_tokens);
                            println!("⏱️  Tempo de processamento: {}ms", index.processing_time_ms);
                            println!();
                            
                            for file_embedding in &index.files {
                                println!("📄 {}", file_embedding.file_path);
                                println!("   📊 Tamanho: {} bytes", file_embedding.file_size);
                                println!("   🔗 Hash: {}", &file_embedding.content_hash[..8]);
                                println!("   📈 Vetor: {} dimensões", file_embedding.embedding_vector.len());
                                println!();
                            }
                            
                            println!("💡 Funcionalidade completa de explicação IA em desenvolvimento...");
                        }
                        Err(_) => {
                            eprintln!("❌ Commit {} não possui análise IA", commit_hash);
                            eprintln!("💡 Use 'cogit index' para ver commits disponíveis");
                        }
                    }
                }
                Err(e) => eprintln!("❌ Erro ao acessar índice IA: {}", e),
            }
        }
        Commands::Index => {
            let cogit_dir = std::path::Path::new(".").join(".cogit");
            match EmbeddingEngine::new(cogit_dir) {
                Ok(engine) => {
                    match engine.list_embedded_commits() {
                        Ok(commits) => {
                            if commits.is_empty() {
                                println!("📋 Nenhum commit com análise IA encontrado");
                                println!("💡 Use 'cogit commit -m \"mensagem\"' para criar commits com IA");
                            } else {
                                println!("🧠 Commits com Análise IA ({}):", commits.len());
                                println!();
                                
                                for commit_hash in commits {
                                    if let Ok(index) = engine.load_embedding_index(&commit_hash) {
                                        println!("🔹 {} ({} arquivo(s))", commit_hash, index.files.len());
                                        println!("   📅 {}", index.created_at.format("%Y-%m-%d %H:%M:%S"));
                                        println!("   🎯 {} tokens | ⏱️ {}ms", index.total_tokens, index.processing_time_ms);
                                        println!();
                                    }
                                }
                                
                                println!("💡 Use 'cogit explain <hash>' para ver detalhes de um commit");
                            }
                        }
                        Err(e) => eprintln!("❌ Erro ao listar commits: {}", e),
                    }
                }
                Err(e) => eprintln!("❌ Erro ao acessar índice IA: {}", e),
            }
        }
    }
}
