use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod cogit;
mod embedding;
mod diff;

use cogit::CogitRepository;
use embedding::EmbeddingEngine;
use diff::DiffEngine;

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
    /// Adiciona arquivos ao staging area
    Add {
        /// Arquivos para adicionar (use "." para todos)
        #[arg(default_value = ".")]
        files: String,
    },
    /// Mostra diferenças entre versões de arquivos
    Diff {
        /// Arquivo específico para mostrar diff (opcional)
        #[arg(long)]
        file: Option<String>,
        /// Mostrar diff do staging area vs working tree
        #[arg(long, default_value = "false")]
        staged: bool,
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
    /// Faz perguntas sobre o código usando IA e embeddings
    Ask {
        /// Pergunta sobre o código ou commits
        #[arg(long = "question", short = 'q')]
        question: String,
        /// Limitar busca a um commit específico (opcional)
        #[arg(long)]
        commit: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            match CogitRepository::init(&path) {
                Ok(_) => println!("Repositório COGIT inicializado em: {}", path.display()),
                Err(e) => eprintln!("Erro ao inicializar repositório: {}", e),
            }
        }
        Commands::Add { files } => {
            match CogitRepository::open(".") {
                Ok(_) => {
                    let cogit_dir = std::path::Path::new(".").join(".cogit");
                    let mut diff_engine = DiffEngine::new(cogit_dir);
                    
                    if files == "." {
                        // Adicionar todos os arquivos
                        match diff_engine.get_status(std::path::Path::new(".")) {
                            Ok(status_list) => {
                                let mut added_count = 0;
                                for file_status in status_list {
                                    match file_status.status {
                                        diff::WorkingTreeStatus::Untracked | 
                                        diff::WorkingTreeStatus::Modified => {
                                            let file_path = std::path::Path::new(&file_status.file_path);
                                            match diff_engine.add_to_staging(file_path) {
                                                Ok(_) => {
                                                    println!("Adicionado: {}", file_status.file_path);
                                                    added_count += 1;
                                                }
                                                Err(e) => eprintln!("Erro ao adicionar {}: {}", file_status.file_path, e),
                                            }
                                        }
                                        _ => {} // Arquivo já staged ou sem mudanças
                                    }
                                }
                                if added_count > 0 {
                                    println!("✅ {} arquivo(s) adicionado(s) ao staging area", added_count);
                                } else {
                                    println!("ℹ️  Nenhuma mudança para adicionar");
                                }
                            }
                            Err(e) => eprintln!("Erro ao verificar status: {}", e),
                        }
                    } else {
                        // Adicionar arquivo específico
                        let file_path = std::path::Path::new(&files);
                        match diff_engine.add_to_staging(file_path) {
                            Ok(_) => println!("✅ Arquivo {} adicionado ao staging area", files),
                            Err(e) => eprintln!("Erro ao adicionar arquivo: {}", e),
                        }
                    }
                }
                Err(e) => eprintln!("Erro: {}", e),
            }
        }
        Commands::Diff { file, staged: _ } => {
            match CogitRepository::open(".") {
                Ok(_) => {
                    let cogit_dir = std::path::Path::new(".").join(".cogit");
                    let diff_engine = DiffEngine::new(cogit_dir);
                    
                    match file {
                        Some(file_path) => {
                            // Mostrar diff de arquivo específico
                            let path = std::path::Path::new(&file_path);
                            match diff_engine.show_file_diff(path) {
                                Ok(_) => {}
                                Err(e) => eprintln!("Erro ao mostrar diff: {}", e),
                            }
                        }
                        None => {
                            // Mostrar diff de todos os arquivos
                            match diff_engine.show_all_diffs(std::path::Path::new(".")) {
                                Ok(_) => {}
                                Err(e) => eprintln!("Erro ao mostrar diffs: {}", e),
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Erro: {}", e),
            }
        }
        Commands::Commit { message, skip_ai } => {
            match CogitRepository::open(".") {
                Ok(mut repo) => {
                    // Verificar se há arquivos no staging area
                    let cogit_dir = std::path::Path::new(".").join(".cogit");
                    let diff_engine = DiffEngine::new(cogit_dir.clone());
                    
                    match diff_engine.load_staging_area() {
                        Ok(staging_area) => {
                            if staging_area.entries.is_empty() {
                                eprintln!("❌ Nenhuma mudança no staging area.");
                                eprintln!("   Use 'cogit add .' para adicionar arquivos antes do commit.");
                                return;
                            }
                            
                            // Processar apenas arquivos staged
                            println!("📦 Criando commit com {} arquivo(s) staged...", staging_area.entries.len());
                            
                            match repo.commit(&message) {
                                Ok(hash) => {
                                    println!("✅ Commit criado: {}", hash);
                                    
                                    // Processar embeddings IA otimizado (apenas patches)
                                    if !skip_ai {
                                        println!("🧠 Iniciando análise semântica otimizada...");
                                        
                                        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
                                            match EmbeddingEngine::new(cogit_dir) {
                                                Ok(mut engine) => {
                                                    engine.set_api_key(api_key);
                                                    
                                                    // TODO: Implementar processamento de patches
                                                    // Por agora, usar o método existente
                                                    match engine.process_commit_embeddings(&hash, std::path::Path::new(".")).await {
                                                        Ok(index) => {
                                                            println!("✅ Análise concluída: {} arquivo(s) processado(s)", index.files.len());
                                                            println!("⏱️  Tempo: {}ms | 🔢 Tokens: {}", index.processing_time_ms, index.total_tokens);
                                                        }
                                                        Err(e) => {
                                                            eprintln!("⚠️  Erro na análise IA: {}", e);
                                                            println!("📝 Commit salvo sem embeddings");
                                                        }
                                                    }
                                                }
                                                Err(e) => eprintln!("❌ Erro ao inicializar motor IA: {}", e),
                                            }
                                        } else {
                                            println!("ℹ️  Para análise IA, defina: export OPENAI_API_KEY=sua_chave");
                                            println!("   Ou use --skip-ai para pular a análise");
                                        }
                                    }
                                    
                                    // Limpar staging area após commit bem-sucedido
                                    let empty_staging = diff::StagingArea {
                                        entries: std::collections::HashMap::new(),
                                        last_updated: chrono::Utc::now(),
                                    };
                                    if let Err(e) = diff_engine.save_staging_area(&empty_staging) {
                                        eprintln!("⚠️  Aviso: Erro ao limpar staging area: {}", e);
                                    }
                                }
                                Err(e) => eprintln!("❌ Erro ao criar commit: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Erro ao acessar staging area: {}", e),
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
                                println!("Nenhum commit encontrado");
                            } else {
                                for commit in commits {
                                    println!("{} - {}", commit.hash, commit.message);
                                    println!("   {}", commit.timestamp);
                                    println!();
                                }
                            }
                        }
                        Err(e) => eprintln!("Erro ao buscar histórico: {}", e),
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
                            println!("📊 {}", status);
                            
                            // Mostrar status detalhado com staging area
                            let cogit_dir = std::path::Path::new(".").join(".cogit");
                            let diff_engine = DiffEngine::new(cogit_dir.clone());
                            
                            match diff_engine.get_status(std::path::Path::new(".")) {
                                Ok(file_statuses) => {
                                    let mut staged_files = Vec::new();
                                    let mut modified_files = Vec::new();
                                    let mut untracked_files = Vec::new();
                                    
                                    for file_status in file_statuses {
                                        match file_status.status {
                                            diff::WorkingTreeStatus::Staged => staged_files.push(file_status.file_path),
                                            diff::WorkingTreeStatus::Modified => modified_files.push(file_status.file_path),
                                            diff::WorkingTreeStatus::Untracked => untracked_files.push(file_status.file_path),
                                            diff::WorkingTreeStatus::Unchanged => {} // Não mostrar arquivos sem mudanças
                                            diff::WorkingTreeStatus::Deleted => {} // TODO: implementar quando necessário
                                        }
                                    }
                                    
                                    if !staged_files.is_empty() {
                                        println!("\n🟢 Mudanças no staging area:");
                                        for file in &staged_files {
                                            println!("  adicionado: {}", file);
                                        }
                                    }
                                    
                                    if !modified_files.is_empty() {
                                        println!("\n🟡 Mudanças não staged:");
                                        for file in &modified_files {
                                            println!("  modificado: {}", file);
                                        }
                                    }
                                    
                                    if !untracked_files.is_empty() {
                                        println!("\n🔴 Arquivos não rastreados:");
                                        for file in &untracked_files {
                                            println!("  {}", file);
                                        }
                                    }
                                    
                                    if staged_files.is_empty() && modified_files.is_empty() && untracked_files.is_empty() {
                                        println!("\n✨ Working tree limpo - nenhuma mudança para commit");
                                    }
                                }
                                Err(e) => eprintln!("Erro ao verificar status detalhado: {}", e),
                            }
                            
                            // Mostrar informações de IA se disponível
                            if let Ok(engine) = EmbeddingEngine::new(cogit_dir) {
                                match engine.list_embedded_commits() {
                                    Ok(commits) => {
                                        if !commits.is_empty() {
                                            println!("\n🤖 Commits com análise IA: {}", commits.len());
                                        }
                                    }
                                    Err(_) => {} // Silenciar erros aqui
                                }
                            }
                        }
                        Err(e) => eprintln!("Erro ao verificar status: {}", e),
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
                            println!("Análise do Commit: {}", commit_hash);
                            println!("Criado em: {}", index.created_at);
                            println!("Arquivos analisados: {}", index.files.len());
                            println!("Tokens processados: {}", index.total_tokens);
                            println!("Tempo de processamento: {}ms", index.processing_time_ms);
                            println!();
                            
                            for file_embedding in &index.files {
                                println!("{}", file_embedding.file_path);
                                println!("   Tamanho: {} bytes", file_embedding.file_size);
                                println!("   Hash: {}", &file_embedding.content_hash[..8]);
                                println!("   Vetor: {} dimensões", file_embedding.embedding_vector.len());
                                println!();
                            }
                            
                            println!("Funcionalidade completa de explicação IA em desenvolvimento...");
                        }
                        Err(_) => {
                            eprintln!("Commit {} não possui análise IA", commit_hash);
                            eprintln!("Use 'cogit index' para ver commits disponíveis");
                        }
                    }
                }
                Err(e) => eprintln!("Erro ao acessar índice IA: {}", e),
            }
        }
        Commands::Index => {
            let cogit_dir = std::path::Path::new(".").join(".cogit");
            match EmbeddingEngine::new(cogit_dir) {
                Ok(engine) => {
                    match engine.list_embedded_commits() {
                        Ok(commits) => {
                            if commits.is_empty() {
                                println!("Nenhum commit com análise IA encontrado");
                                println!("Use 'cogit commit -m \"mensagem\"' para criar commits com IA");
                            } else {
                                println!("Commits com Análise IA ({}):", commits.len());
                                println!();
                                
                                for commit_hash in commits {
                                    if let Ok(index) = engine.load_embedding_index(&commit_hash) {
                                        println!("{} ({} arquivo(s))", commit_hash, index.files.len());
                                        println!("   {}", index.created_at.format("%Y-%m-%d %H:%M:%S"));
                                        println!("   {} tokens | {}ms", index.total_tokens, index.processing_time_ms);
                                        println!();
                                    }
                                }
                                
                                println!("Use 'cogit explain <hash>' para ver detalhes de um commit");
                            }
                        }
                        Err(e) => eprintln!("Erro ao listar commits: {}", e),
                    }
                }
                Err(e) => eprintln!("Erro ao acessar índice IA: {}", e),
            }
        }
        Commands::Ask { question, commit } => {
            let cogit_dir = std::path::Path::new(".").join(".cogit");
            match EmbeddingEngine::new(cogit_dir) {
                Ok(mut engine) => {
                    // Obter chave da API via variável de ambiente
                    if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
                        engine.set_api_key(api_key);
                        
                        match engine.ask_question(&question, commit.as_deref()).await {
                            Ok(answer) => {
                                println!("Resposta:");
                                println!("{}", answer);
                            }
                            Err(e) => eprintln!("Erro ao processar pergunta: {}", e),
                        }
                    } else {
                        eprintln!("Para usar IA, defina: export OPENAI_API_KEY=sua_chave");
                    }
                }
                Err(e) => eprintln!("Erro ao acessar sistema IA: {}", e),
            }
        }
    }
}
