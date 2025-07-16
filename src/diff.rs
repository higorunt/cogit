use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::cogit::CogitError;

/// Representa uma linha em um diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    pub line_number: usize,
    pub content: String,
    pub change_type: LineChangeType,
}

/// Tipo de mudança em uma linha
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineChangeType {
    Added,    // Linha adicionada (+)
    Removed,  // Linha removida (-)
    Context,  // Linha de contexto (sem mudança)
}

/// Representa um hunk (bloco de mudanças) em um diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub lines: Vec<DiffLine>,
}

/// Representa um diff completo de um arquivo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    pub file_path: String,
    pub old_hash: Option<String>,  // None se arquivo é novo
    pub new_hash: String,
    pub change_type: FileChangeType,
    pub hunks: Vec<DiffHunk>,
    pub patch_content: String,  // Conteúdo textual do patch
    pub created_at: DateTime<Utc>,
}

/// Tipo de mudança do arquivo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileChangeType {
    Added,     // Arquivo novo
    Modified,  // Arquivo modificado
    Deleted,   // Arquivo removido
    Renamed,   // Arquivo renomeado (futura implementação)
}

/// Status de um arquivo no working directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStatus {
    pub file_path: String,
    pub working_tree_hash: Option<String>,  // Hash do arquivo atual
    pub index_hash: Option<String>,         // Hash no staging area
    pub head_hash: Option<String>,          // Hash no último commit
    pub status: WorkingTreeStatus,
}

/// Status do arquivo no working tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkingTreeStatus {
    Untracked,    // Arquivo não rastreado
    Modified,     // Modificado desde último commit
    Staged,       // Adicionado ao staging area
    Deleted,      // Deletado
    Unchanged,    // Sem mudanças
}

/// Staging area (index) - similar ao git index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagingArea {
    pub entries: HashMap<String, StagingEntry>,
    pub last_updated: DateTime<Utc>,
}

/// Entrada no staging area
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagingEntry {
    pub file_path: String,
    pub content_hash: String,
    pub file_size: u64,
    pub staged_at: DateTime<Utc>,
}

/// Motor de diff - implementa algoritmos de comparação
pub struct DiffEngine {
    cogit_dir: PathBuf,
}

impl DiffEngine {
    /// Cria novo motor de diff
    pub fn new(cogit_dir: PathBuf) -> Self {
        Self { cogit_dir }
    }
    
    /// Calcula diff entre duas versões de um arquivo
    pub fn calculate_file_diff(
        &self,
        file_path: &Path,
        old_content: Option<&str>,
        new_content: &str,
    ) -> Result<FileDiff, CogitError> {
        let old_hash = old_content.map(|content| 
            crate::cogit::CogitRepository::calculate_hash(content.as_bytes())
        );
        let new_hash = crate::cogit::CogitRepository::calculate_hash(new_content.as_bytes());
        
        let change_type = match old_content {
            None => FileChangeType::Added,
            Some(old) if old == new_content => return Err(CogitError::IoError(
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Arquivo sem mudanças")
            )),
            Some(_) => FileChangeType::Modified,
        };
        
        let hunks = if let Some(old) = old_content {
            self.calculate_hunks(old, new_content)?
        } else {
            // Arquivo novo - todo conteúdo é uma adição
            vec![self.create_addition_hunk(new_content)]
        };
        
        let patch_content = self.generate_patch_content(&hunks, file_path)?;
        
        Ok(FileDiff {
            file_path: file_path.to_string_lossy().to_string(),
            old_hash,
            new_hash,
            change_type,
            hunks,
            patch_content,
            created_at: Utc::now(),
        })
    }
    
    /// Calcula hunks (blocos de mudanças) usando algoritmo de diff simples
    fn calculate_hunks(&self, old_content: &str, new_content: &str) -> Result<Vec<DiffHunk>, CogitError> {
        let old_lines: Vec<&str> = old_content.lines().collect();
        let new_lines: Vec<&str> = new_content.lines().collect();
        
        // Implementação simples de diff - algoritmo Myers básico
        let mut hunks = Vec::new();
        let mut old_idx = 0;
        let mut new_idx = 0;
        
        while old_idx < old_lines.len() || new_idx < new_lines.len() {
            let mut hunk_lines = Vec::new();
            let hunk_old_start = old_idx + 1;
            let hunk_new_start = new_idx + 1;
            let mut hunk_old_count = 0;
            let mut hunk_new_count = 0;
            
            // Adicionar contexto antes das mudanças
            let context_before = 3;
            let context_start = old_idx.saturating_sub(context_before);
            for i in context_start..old_idx {
                if i < old_lines.len() {
                    hunk_lines.push(DiffLine {
                        line_number: i + 1,
                        content: old_lines[i].to_string(),
                        change_type: LineChangeType::Context,
                    });
                    hunk_old_count += 1;
                    hunk_new_count += 1;
                }
            }
            
            // Detectar mudanças (implementação simplificada)
            while old_idx < old_lines.len() && new_idx < new_lines.len() {
                if old_lines[old_idx] == new_lines[new_idx] {
                    // Linha igual - contexto
                    hunk_lines.push(DiffLine {
                        line_number: old_idx + 1,
                        content: old_lines[old_idx].to_string(),
                        change_type: LineChangeType::Context,
                    });
                    hunk_old_count += 1;
                    hunk_new_count += 1;
                    old_idx += 1;
                    new_idx += 1;
                } else {
                    // Linhas diferentes - remoção + adição
                    hunk_lines.push(DiffLine {
                        line_number: old_idx + 1,
                        content: old_lines[old_idx].to_string(),
                        change_type: LineChangeType::Removed,
                    });
                    hunk_old_count += 1;
                    old_idx += 1;
                    
                    hunk_lines.push(DiffLine {
                        line_number: new_idx + 1,
                        content: new_lines[new_idx].to_string(),
                        change_type: LineChangeType::Added,
                    });
                    hunk_new_count += 1;
                    new_idx += 1;
                    break;
                }
            }
            
            // Processar linhas restantes
            while old_idx < old_lines.len() {
                hunk_lines.push(DiffLine {
                    line_number: old_idx + 1,
                    content: old_lines[old_idx].to_string(),
                    change_type: LineChangeType::Removed,
                });
                hunk_old_count += 1;
                old_idx += 1;
            }
            
            while new_idx < new_lines.len() {
                hunk_lines.push(DiffLine {
                    line_number: new_idx + 1,
                    content: new_lines[new_idx].to_string(),
                    change_type: LineChangeType::Added,
                });
                hunk_new_count += 1;
                new_idx += 1;
            }
            
            if !hunk_lines.is_empty() {
                hunks.push(DiffHunk {
                    old_start: hunk_old_start,
                    old_count: hunk_old_count,
                    new_start: hunk_new_start,
                    new_count: hunk_new_count,
                    lines: hunk_lines,
                });
            }
            
            break; // Por agora, um hunk por vez (simplificado)
        }
        
        Ok(hunks)
    }
    
    /// Cria hunk para arquivo completamente novo
    fn create_addition_hunk(&self, content: &str) -> DiffHunk {
        let lines: Vec<&str> = content.lines().collect();
        let diff_lines: Vec<DiffLine> = lines
            .iter()
            .enumerate()
            .map(|(idx, line)| DiffLine {
                line_number: idx + 1,
                content: line.to_string(),
                change_type: LineChangeType::Added,
            })
            .collect();
        
        DiffHunk {
            old_start: 0,
            old_count: 0,
            new_start: 1,
            new_count: lines.len(),
            lines: diff_lines,
        }
    }
    
    /// Gera conteúdo do patch no formato unified diff
    fn generate_patch_content(&self, hunks: &[DiffHunk], file_path: &Path) -> Result<String, CogitError> {
        let mut patch = String::new();
        
        // Header do patch
        patch.push_str(&format!("--- a/{}\n", file_path.display()));
        patch.push_str(&format!("+++ b/{}\n", file_path.display()));
        
        for hunk in hunks {
            // Header do hunk
            patch.push_str(&format!(
                "@@ -{},{} +{},{} @@\n",
                hunk.old_start, hunk.old_count,
                hunk.new_start, hunk.new_count
            ));
            
            // Linhas do hunk
            for line in &hunk.lines {
                let prefix = match line.change_type {
                    LineChangeType::Added => "+",
                    LineChangeType::Removed => "-",
                    LineChangeType::Context => " ",
                };
                patch.push_str(&format!("{}{}\n", prefix, line.content));
            }
        }
        
        Ok(patch)
    }
    
    /// Carrega staging area do disco
    pub fn load_staging_area(&self) -> Result<StagingArea, CogitError> {
        let index_path = self.cogit_dir.join("index.json");
        
        if !index_path.exists() {
            return Ok(StagingArea {
                entries: HashMap::new(),
                last_updated: Utc::now(),
            });
        }
        
        let content = fs::read_to_string(index_path)?;
        let staging_area: StagingArea = serde_json::from_str(&content)?;
        Ok(staging_area)
    }
    
    /// Salva staging area no disco
    pub fn save_staging_area(&self, staging_area: &StagingArea) -> Result<(), CogitError> {
        let index_path = self.cogit_dir.join("index.json");
        let content = serde_json::to_string_pretty(staging_area)?;
        fs::write(index_path, content)?;
        Ok(())
    }
    
    /// Adiciona arquivo ao staging area
    pub fn add_to_staging(&mut self, file_path: &Path) -> Result<(), CogitError> {
        if !file_path.exists() {
            return Err(CogitError::IoError(
                std::io::Error::new(std::io::ErrorKind::NotFound, "Arquivo não encontrado")
            ));
        }
        
        let content = fs::read_to_string(file_path)?;
        let content_hash = crate::cogit::CogitRepository::calculate_hash(content.as_bytes());
        let metadata = fs::metadata(file_path)?;
        
        let mut staging_area = self.load_staging_area()?;
        
        let entry = StagingEntry {
            file_path: file_path.to_string_lossy().to_string(),
            content_hash,
            file_size: metadata.len(),
            staged_at: Utc::now(),
        };
        
        staging_area.entries.insert(
            file_path.to_string_lossy().to_string(),
            entry
        );
        staging_area.last_updated = Utc::now();
        
        self.save_staging_area(&staging_area)?;
        Ok(())
    }
    
    /// Lista status de todos os arquivos
    pub fn get_status(&self, root_path: &Path) -> Result<Vec<FileStatus>, CogitError> {
        let mut status_list = Vec::new();
        let staging_area = self.load_staging_area()?;
        
        // Obter arquivos do último commit (HEAD) se existir
        let head_files = self.get_head_files()?;
        
        // Percorrer arquivos no working directory
        for entry in fs::read_dir(root_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && !path.starts_with(root_path.join(".cogit")) {
                // Normalizar nome do arquivo para comparação (remover ./ do início)
                let file_path = if let Ok(relative) = path.strip_prefix(root_path) {
                    relative.to_string_lossy().to_string()
                } else {
                    path.to_string_lossy().to_string()
                };
                
                // Calcular hash atual
                let current_content = fs::read_to_string(&path)?;
                let working_tree_hash = Some(
                    crate::cogit::CogitRepository::calculate_hash(current_content.as_bytes())
                );
                
                // Verificar se está no staging
                let index_hash = staging_area.entries.get(&file_path)
                    .map(|entry| entry.content_hash.clone());
                
                // Verificar hash no HEAD
                let head_hash = head_files.get(&file_path).cloned();
                
                // Determinar status baseado em staging, working tree e HEAD
                let status = match (&index_hash, &head_hash, &working_tree_hash) {
                    // Arquivo staged (seja novo ou modificado)
                    (Some(staged_hash), head_hash_opt, Some(work_hash)) 
                        if staged_hash == work_hash => WorkingTreeStatus::Staged,
                    
                    // Arquivo modificado após staging
                    (Some(_), _, _) => WorkingTreeStatus::Modified,
                    
                    // Arquivo não está staged
                    (None, Some(head_hash_val), Some(work_hash)) => {
                        if head_hash_val == work_hash {
                            WorkingTreeStatus::Unchanged
                        } else {
                            WorkingTreeStatus::Modified
                        }
                    }
                    
                    // Arquivo novo (não tracked em nenhum commit)
                    (None, None, _) => WorkingTreeStatus::Untracked,
                    
                    _ => WorkingTreeStatus::Untracked,
                };
                
                status_list.push(FileStatus {
                    file_path,
                    working_tree_hash,
                    index_hash,
                    head_hash,
                    status,
                });
            }
        }
        
        Ok(status_list)
    }
    
    /// Obtém arquivos do último commit (HEAD)
    fn get_head_files(&self) -> Result<HashMap<String, String>, CogitError> {
        use crate::cogit::{CogitRepository, TreeEntry};
        
        let mut head_files = HashMap::new();
        
        // Tentar abrir repositório e obter último commit
        let repo = CogitRepository::open(self.cogit_dir.parent().unwrap_or(std::path::Path::new(".")))?;
        
        if let Ok(commits) = repo.log() {
            if let Some(last_commit) = commits.first() {
                // Carregar tree do último commit
                if let Ok(tree_data) = self.load_object(&last_commit.tree_hash) {
                    if let Ok(tree_entries) = serde_json::from_slice::<Vec<TreeEntry>>(&tree_data) {
                        for entry in tree_entries {
                            if entry.is_file {
                                head_files.insert(entry.name, entry.hash);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(head_files)
    }
    
    /// Carrega um objeto do armazenamento (helper method)
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
    
    /// Mostra diff de um arquivo específico
    pub fn show_file_diff(&self, file_path: &Path) -> Result<(), CogitError> {
        if !file_path.exists() {
            return Err(CogitError::IoError(
                std::io::Error::new(std::io::ErrorKind::NotFound, "Arquivo não encontrado")
            ));
        }
        
        let current_content = fs::read_to_string(file_path)?;
        
        // Por agora, vamos comparar com "arquivo vazio" para mostrar todo conteúdo como adição
        // TODO: Implementar comparação com última versão commitada
        let diff = self.calculate_file_diff(file_path, None, &current_content)?;
        
        println!("diff --git a/{} b/{}", file_path.display(), file_path.display());
        println!("new file mode 100644");
        println!("index 0000000..{}", &diff.new_hash[..7]);
        println!("{}", diff.patch_content);
        
        Ok(())
    }
    
    /// Mostra diffs de todos os arquivos não staged
    pub fn show_all_diffs(&self, root_path: &Path) -> Result<(), CogitError> {
        let status_list = self.get_status(root_path)?;
        let mut has_diffs = false;
        
        for file_status in status_list {
            match file_status.status {
                WorkingTreeStatus::Modified | WorkingTreeStatus::Untracked => {
                    if has_diffs {
                        println!(); // Linha em branco entre arquivos
                    }
                    self.show_file_diff(std::path::Path::new(&file_status.file_path))?;
                    has_diffs = true;
                }
                _ => {}
            }
        }
        
        if !has_diffs {
            println!("Nenhuma mudança para mostrar");
        }
        
        Ok(())
    }
} 