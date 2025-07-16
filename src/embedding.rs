use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
        
        // Ignorar arquivos de documentação e configuração específicos
        let ignored_files = [
            "README.md", "CHANGELOG.md", "LICENSE", "LICENSE.txt",
            "CONTRIBUTING.md", "CODE_OF_CONDUCT.md", "SECURITY.md",
            "GUIA_DESENVOLVIMENTO.md", "CONTEXTO_CHATGPT.md", 
            "TESTE_FUNCIONALIDADES.md", "STATUS_SEMINARIO.md"
        ];
        
        if ignored_files.contains(&file_name) {
            return false;
        }
        
        // Lista de extensões de código fonte (excluindo documentação)
        let code_extensions = [
            ".rs", ".py", ".js", ".ts", ".java", ".cpp", ".c", ".h",
            ".go", ".rb", ".php", ".swift", ".kt", ".scala", ".clj",
            ".sh", ".bash", ".sql", ".html", ".css", ".json", ".xml",
            ".yaml", ".yml", ".toml"
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
} 