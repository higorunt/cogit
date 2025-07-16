use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::cogit::CogitError;

/// Configuração para API OpenAI
#[derive(Debug, Clone)]
pub struct OpenAIConfig {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(), // Será preenchido via variável de ambiente
            model: "text-embedding-3-small".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }
}

/// Representa um embedding de arquivo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEmbedding {
    pub file_path: String,
    pub content_hash: String,
    pub embedding_vector: Vec<f32>,
    pub change_type: ChangeType,
    pub file_size: u64,
    pub created_at: DateTime<Utc>,
}

/// Tipo de mudança no arquivo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
}

/// Índice de embeddings para um commit específico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingIndex {
    pub commit_hash: String,
    pub files: Vec<FileEmbedding>,
    pub total_tokens: u32,
    pub processing_time_ms: u64,
    pub created_at: DateTime<Utc>,
}

/// Request para API OpenAI Embeddings
#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    input: String,
    model: String,
}

/// Response da API OpenAI Embeddings
#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

#[derive(Debug, Deserialize)]
struct Usage {
    total_tokens: u32,
}

/// Request para API OpenAI Chat Completion
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// Response da API OpenAI Chat Completion
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

/// Motor principal de embeddings
pub struct EmbeddingEngine {
    config: OpenAIConfig,
    client: Client,
    cogit_dir: PathBuf,
}

impl EmbeddingEngine {
    /// Cria novo motor de embeddings
    pub fn new(cogit_dir: PathBuf) -> Result<Self, CogitError> {
        let config = OpenAIConfig::default();
        let client = Client::new();
        
        // Cria diretório de índices se não existir
        let index_dir = cogit_dir.join("index");
        fs::create_dir_all(&index_dir)?;
        
        Ok(Self {
            config,
            client,
            cogit_dir,
        })
    }
    
    /// Define a chave da API OpenAI
    pub fn set_api_key(&mut self, api_key: String) {
        self.config.api_key = api_key;
    }
    
    /// Analisa arquivos modificados e retorna lista de caminhos válidos
    pub fn analyze_modified_files(&self, root_path: &Path) -> Result<Vec<PathBuf>, CogitError> {
        let mut valid_files = Vec::new();
        
        // Por enquanto, vamos analisar todos os arquivos no diretório raiz
        // Em uma implementação futura, isso seria baseado em git diff
        for entry in fs::read_dir(root_path)? {
            let entry = entry?;
            let path = entry.path();
            
            // Filtrar apenas arquivos de código válidos
            if self.is_code_file(&path) {
                valid_files.push(path);
            }
        }
        
        Ok(valid_files)
    }
    
    /// Verifica se é um arquivo de código válido para embedding
    fn is_code_file(&self, path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }
        
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        // Ignorar arquivos ocultos e diretório .cogit
        if file_name.starts_with('.') {
            return false;
        }
        
        // Ignorar apenas arquivos de documentação específicos (manter README.md para IA)
        let ignored_files = [
            "CHANGELOG.md", "LICENSE", "LICENSE.txt",
            "CONTRIBUTING.md", "CODE_OF_CONDUCT.md", "SECURITY.md",
            "GUIA_DESENVOLVIMENTO.md", "CONTEXTO_CHATGPT.md", 
            "TESTE_FUNCIONALIDADES.md", "STATUS_SEMINARIO.md"
        ];
        
        if ignored_files.contains(&file_name) {
            return false;
        }
        
        // Lista de extensões válidas para análise IA (código + documentação relevante)
        let code_extensions = [
            ".rs", ".py", ".js", ".ts", ".java", ".cpp", ".c", ".h",
            ".go", ".rb", ".php", ".swift", ".kt", ".scala", ".clj",
            ".sh", ".bash", ".sql", ".html", ".css", ".json", ".xml",
            ".yaml", ".yml", ".toml", ".md", ".txt"
        ];
        
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            let ext_with_dot = format!(".{}", extension);
            return code_extensions.contains(&ext_with_dot.as_str());
        }
        
        false
    }
    
    /// Gera embedding para um arquivo usando OpenAI API
    pub async fn generate_file_embedding(&self, file_path: &Path) -> Result<FileEmbedding, CogitError> {
        // Ler conteúdo do arquivo
        let content = fs::read_to_string(file_path)
            .map_err(|e| CogitError::IoError(e))?;
        
        // Calcular hash do conteúdo
        let content_hash = crate::cogit::CogitRepository::calculate_hash(content.as_bytes());
        
        // Gerar embedding via OpenAI
        let embedding_vector = self.call_openai_embedding(&content).await?;
        
        // Obter metadados do arquivo
        let metadata = fs::metadata(file_path)?;
        let file_size = metadata.len();
        
        Ok(FileEmbedding {
            file_path: file_path.to_string_lossy().to_string(),
            content_hash,
            embedding_vector,
            change_type: ChangeType::Modified, // Por enquanto, assumir modificado
            file_size,
            created_at: Utc::now(),
        })
    }
    
    /// Chama a API OpenAI para gerar embedding
    async fn call_openai_embedding(&self, content: &str) -> Result<Vec<f32>, CogitError> {
        if self.config.api_key.is_empty() {
            return Err(CogitError::IoError(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Chave da API OpenAI não configurada"
                )
            ));
        }
        
        let request = EmbeddingRequest {
            input: content.to_string(),
            model: self.config.model.clone(),
        };
        
        let response = self.client
            .post(&format!("{}/embeddings", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| CogitError::IoError(
                std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
            ))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Erro desconhecido".to_string());
            return Err(CogitError::IoError(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Erro da API OpenAI: {}", error_text)
                )
            ));
        }
        
        let embedding_response: EmbeddingResponse = response
            .json()
            .await
            .map_err(|e| CogitError::IoError(
                std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
            ))?;
        
        if let Some(embedding_data) = embedding_response.data.first() {
            Ok(embedding_data.embedding.clone())
        } else {
            Err(CogitError::IoError(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Resposta da API OpenAI vazia"
                )
            ))
        }
    }
    
    /// Processa todos os arquivos modificados e gera índice de embeddings
    pub async fn process_commit_embeddings(
        &self,
        commit_hash: &str,
        root_path: &Path,
    ) -> Result<EmbeddingIndex, CogitError> {
        let start_time = std::time::Instant::now();
        
        // Analisar arquivos modificados
        let files_to_process = self.analyze_modified_files(root_path)?;
        
        let mut file_embeddings = Vec::new();
        let mut total_tokens = 0u32;
        
        // Processar cada arquivo
        for file_path in files_to_process {
            println!("Processando: {}", file_path.display());
            
            match self.generate_file_embedding(&file_path).await {
                Ok(embedding) => {
                    file_embeddings.push(embedding);
                    total_tokens += 1000; // Estimativa - seria obtida da resposta da API
                }
                Err(e) => {
                    eprintln!("⚠️  Erro ao processar {}: {}", file_path.display(), e);
                    // Continue processando outros arquivos
                }
            }
        }
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        let index = EmbeddingIndex {
            commit_hash: commit_hash.to_string(),
            files: file_embeddings,
            total_tokens,
            processing_time_ms: processing_time,
            created_at: Utc::now(),
        };
        
        // Salvar índice em disco
        self.save_embedding_index(&index)?;
        
        Ok(index)
    }
    
    /// Salva índice de embeddings em disco
    fn save_embedding_index(&self, index: &EmbeddingIndex) -> Result<(), CogitError> {
        let index_path = self.cogit_dir
            .join("index")
            .join(format!("{}.json", index.commit_hash));
        
        let json_content = serde_json::to_string_pretty(index)?;
        fs::write(index_path, json_content)?;
        
        Ok(())
    }
    
    /// Carrega índice de embeddings do disco
    pub fn load_embedding_index(&self, commit_hash: &str) -> Result<EmbeddingIndex, CogitError> {
        let index_path = self.cogit_dir
            .join("index")
            .join(format!("{}.json", commit_hash));
        
        if !index_path.exists() {
            return Err(CogitError::InvalidHash);
        }
        
        let json_content = fs::read_to_string(index_path)?;
        let index: EmbeddingIndex = serde_json::from_str(&json_content)?;
        
        Ok(index)
    }
    
    /// Lista todos os commits que possuem embeddings
    pub fn list_embedded_commits(&self) -> Result<Vec<String>, CogitError> {
        let index_dir = self.cogit_dir.join("index");
        
        if !index_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut commits = Vec::new();
        
        for entry in fs::read_dir(index_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |e| e == "json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    commits.push(stem.to_string());
                }
            }
        }
        
        commits.sort();
        Ok(commits)
    }
    
    /// Calcula a similaridade de cosseno entre dois vetores
    fn cosine_similarity(&self, vec1: &[f32], vec2: &[f32]) -> f32 {
        if vec1.len() != vec2.len() {
            return 0.0;
        }
        
        let dot_product: f32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
        let magnitude1: f32 = vec1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let magnitude2: f32 = vec2.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if magnitude1 == 0.0 || magnitude2 == 0.0 {
            return 0.0;
        }
        
        dot_product / (magnitude1 * magnitude2)
    }
    
    /// Busca os embeddings mais similares à pergunta
    async fn find_relevant_embeddings(&self, question: &str, commit_filter: Option<&str>) -> Result<Vec<(String, FileEmbedding, f32)>, CogitError> {
        // Gerar embedding da pergunta
        let question_embedding = self.call_openai_embedding(question).await?;
        
        // Obter lista de commits a buscar
        let commits_to_search = if let Some(commit_hash) = commit_filter {
            vec![commit_hash.to_string()]
        } else {
            self.list_embedded_commits()?
        };
        
        let mut results = Vec::new();
        
        // Buscar em cada commit
        for commit_hash in commits_to_search {
            if let Ok(index) = self.load_embedding_index(&commit_hash) {
                for file_embedding in index.files {
                    let similarity = self.cosine_similarity(&question_embedding, &file_embedding.embedding_vector);
                    if similarity > 0.1 { // Threshold mínimo
                        results.push((commit_hash.clone(), file_embedding, similarity));
                    }
                }
            }
        }
        
        // Ordenar por similaridade (maior para menor)
        results.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        
        // Retornar apenas os mais relevantes (máximo 5)
        results.truncate(5);
        
        Ok(results)
    }
    
    /// Constrói contexto para a resposta baseado nos embeddings mais relevantes
    async fn build_context(&self, relevant_embeddings: &[(String, FileEmbedding, f32)]) -> Result<String, CogitError> {
        let mut context = String::new();
        
        context.push_str("Contexto dos arquivos relevantes encontrados:\n\n");
        
        for (commit_hash, file_embedding, similarity) in relevant_embeddings {
            context.push_str(&format!("Arquivo: {} (Commit: {}, Similaridade: {:.2})\n", 
                file_embedding.file_path, 
                &commit_hash[..8],
                similarity
            ));
            
            // Tentar carregar o conteúdo atual do arquivo se ainda existe
            if let Ok(content) = std::fs::read_to_string(&file_embedding.file_path) {
                context.push_str("Conteúdo:\n```\n");
                context.push_str(&content);
                context.push_str("\n```\n\n");
            } else {
                context.push_str("(Arquivo não encontrado ou foi removido)\n\n");
            }
        }
        
        Ok(context)
    }
    
    /// Chama a API OpenAI Chat Completion
    async fn call_openai_chat(&self, messages: Vec<ChatMessage>) -> Result<String, CogitError> {
        if self.config.api_key.is_empty() {
            return Err(CogitError::IoError(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Chave da API OpenAI não configurada"
                )
            ));
        }
        
        let request = ChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages,
            temperature: 0.7,
            max_tokens: 1000,
        };
        
        let response = self.client
            .post(&format!("{}/chat/completions", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| CogitError::IoError(
                std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
            ))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Erro desconhecido".to_string());
            return Err(CogitError::IoError(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Erro da API OpenAI: {}", error_text)
                )
            ));
        }
        
        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| CogitError::IoError(
                std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
            ))?;
        
        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(CogitError::IoError(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Resposta da API OpenAI vazia"
                )
            ))
        }
    }
    
    /// Função principal: faz pergunta sobre o código usando embeddings e IA
    pub async fn ask_question(&self, question: &str, commit_filter: Option<&str>) -> Result<String, CogitError> {
        println!("🔍 Buscando informações relevantes...");
        
        // Buscar embeddings mais similares à pergunta
        let relevant_embeddings = self.find_relevant_embeddings(question, commit_filter).await?;
        
        if relevant_embeddings.is_empty() {
            return Ok("Não encontrei informações relevantes para responder sua pergunta. Certifique-se de que existem commits com análise IA.".to_string());
        }
        
        println!("📋 Encontrados {} arquivo(s) relevante(s)", relevant_embeddings.len());
        
        // Construir contexto com os arquivos mais relevantes
        let context = self.build_context(&relevant_embeddings).await?;
        
        // Preparar mensagens para o chat
        let system_message = ChatMessage {
            role: "system".to_string(),
            content: "Você é um assistente especializado em análise de código. Use o contexto fornecido para responder perguntas sobre o código de forma clara e útil. Se a pergunta não puder ser respondida com o contexto, diga isso claramente.".to_string(),
        };
        
        let context_message = ChatMessage {
            role: "user".to_string(),
            content: format!("{}\n\nPergunta: {}", context, question),
        };
        
        println!("🤖 Processando resposta com IA...");
        
        // Obter resposta da IA
        let response = self.call_openai_chat(vec![system_message, context_message]).await?;
        
        Ok(response)
    }
} 