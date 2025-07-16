use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Estrutura principal do repositório COGIT
pub struct CogitRepository {
    root_path: PathBuf,
    cogit_dir: PathBuf,
}

/// Representa um commit no sistema COGIT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub hash: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub parent: Option<String>,
    pub tree_hash: String,
}

/// Representa uma entrada na árvore de arquivos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeEntry {
    pub name: String,
    pub hash: String,
    pub is_file: bool,
}

/// Erros específicos do COGIT
#[derive(Debug)]
pub enum CogitError {
    IoError(io::Error),
    NotARepository,
    InvalidHash,
    SerializationError(serde_json::Error),
}

impl std::fmt::Display for CogitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CogitError::IoError(e) => write!(f, "Erro de I/O: {}", e),
            CogitError::NotARepository => write!(f, "Não é um repositório COGIT válido"),
            CogitError::InvalidHash => write!(f, "Hash inválido"),
            CogitError::SerializationError(e) => write!(f, "Erro de serialização: {}", e),
        }
    }
}

impl From<io::Error> for CogitError {
    fn from(error: io::Error) -> Self {
        CogitError::IoError(error)
    }
}

impl From<serde_json::Error> for CogitError {
    fn from(error: serde_json::Error) -> Self {
        CogitError::SerializationError(error)
    }
}

impl CogitRepository {
    /// Inicializa um novo repositório COGIT
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self, CogitError> {
        let root_path = path.as_ref().to_path_buf();
        let cogit_dir = root_path.join(".cogit");

        // Cria a estrutura de diretórios
        fs::create_dir_all(&cogit_dir)?;
        fs::create_dir_all(cogit_dir.join("objects"))?;
        fs::create_dir_all(cogit_dir.join("refs"))?;
        fs::create_dir_all(cogit_dir.join("refs").join("heads"))?;

        // Cria arquivo HEAD inicial
        fs::write(cogit_dir.join("HEAD"), "ref: refs/heads/main\n")?;

        // Cria arquivo de configuração inicial
        let config = serde_json::json!({
            "version": "0.1.0",
            "created": Utc::now(),
            "description": "Repositório COGIT - Cognition Git"
        });
        fs::write(cogit_dir.join("config.json"), serde_json::to_string_pretty(&config)?)?;

        Ok(Self { root_path, cogit_dir })
    }

    /// Abre um repositório COGIT existente
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, CogitError> {
        let root_path = path.as_ref().to_path_buf();
        let cogit_dir = root_path.join(".cogit");

        if !cogit_dir.exists() {
            return Err(CogitError::NotARepository);
        }

        Ok(Self { root_path, cogit_dir })
    }

    /// Calcula o hash SHA-256 de um conteúdo
    pub fn calculate_hash(content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }

    /// Armazena um objeto no sistema content-addressable
    fn store_object(&self, content: &[u8]) -> Result<String, CogitError> {
        let hash = Self::calculate_hash(content);
        let object_dir = self.cogit_dir.join("objects").join(&hash[..2]);
        fs::create_dir_all(&object_dir)?;
        
        let object_path = object_dir.join(&hash[2..]);
        fs::write(object_path, content)?;
        
        Ok(hash)
    }

    /// Cria uma árvore a partir do diretório atual
    fn create_tree(&self) -> Result<String, CogitError> {
        let mut entries = Vec::new();
        
        // Percorre os arquivos no diretório raiz (implementação simplificada)
        for entry in fs::read_dir(&self.root_path)? {
            let entry = entry?;
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            
            // Ignora o diretório .cogit e .git
            if name.starts_with('.') {
                continue;
            }
            
            if path.is_file() {
                let content = fs::read(&path)?;
                let hash = self.store_object(&content)?;
                entries.push(TreeEntry {
                    name,
                    hash,
                    is_file: true,
                });
            }
        }
        
        let tree_content = serde_json::to_vec(&entries)?;
        self.store_object(&tree_content)
    }

    /// Cria um novo commit
    pub fn commit(&mut self, message: &str) -> Result<String, CogitError> {
        let tree_hash = self.create_tree()?;
        
        // Busca o commit pai (se existir)
        let parent = self.get_current_commit_hash().ok();
        
        let commit = Commit {
            hash: String::new(), // Temporário
            message: message.to_string(),
            timestamp: Utc::now(),
            parent,
            tree_hash,
        };
        
        // Serializa o commit sem o hash para calcular o hash correto
        let commit_content = serde_json::to_vec(&commit)?;
        
        // Armazena o commit com o hash calculado
        let stored_hash = self.store_object(&commit_content)?;
        
        // Atualiza a referência HEAD
        fs::write(self.cogit_dir.join("refs").join("heads").join("main"), &stored_hash)?;
        
        Ok(stored_hash)
    }

    /// Obtém o hash do commit atual
    fn get_current_commit_hash(&self) -> Result<String, CogitError> {
        let head_path = self.cogit_dir.join("refs").join("heads").join("main");
        if head_path.exists() {
            Ok(fs::read_to_string(head_path)?.trim().to_string())
        } else {
            Err(CogitError::NotARepository)
        }
    }

    /// Carrega um objeto do armazenamento
    fn load_object(&self, hash: &str) -> Result<Vec<u8>, CogitError> {
        let object_path = self.cogit_dir
            .join("objects")
            .join(&hash[..2])
            .join(&hash[2..]);
        
        if !object_path.exists() {
            return Err(CogitError::InvalidHash);
        }
        
        Ok(fs::read(object_path)?)
    }

    /// Mostra o histórico de commits
    pub fn log(&self) -> Result<Vec<Commit>, CogitError> {
        let mut commits = Vec::new();
        let mut current_hash = self.get_current_commit_hash().ok();
        
        while let Some(hash) = current_hash {
            let commit_data = self.load_object(&hash)?;
            let commit: Commit = serde_json::from_slice(&commit_data)?;
            current_hash = commit.parent.clone();
            commits.push(commit);
        }
        
        Ok(commits)
    }

    /// Mostra o status atual do repositório
    pub fn status(&self) -> Result<String, CogitError> {
        let commit_count = self.log()?.len();
        Ok(format!("Repositório COGIT com {} commit(s)", commit_count))
    }
} 