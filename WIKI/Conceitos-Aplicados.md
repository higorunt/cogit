# Conceitos de Linguagens de Programação Aplicados

## 🎯 Sistemas de Tipos e Segurança

### Tipos Fortes e Estáticos

Rust utiliza um sistema de tipos robusto que previne muitos erros em tempo de compilação:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEmbedding {
    pub file_path: String,
    pub embedding_vector: Vec<f32>,
    pub change_type: ChangeType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
}
```

**Vantagens demonstradas:**
- **Type safety**: Impossível confundir um `String` com um `Vec<f32>`
- **Enum safety**: Compilador força tratamento de todos os casos
- **Null safety**: `Option<T>` ao invés de null pointers
- **Error handling**: `Result<T, E>` ao invés de exceptions

### Tratamento de Erros Explícito

```rust
pub enum CogitError {
    IoError(io::Error),
    NotARepository,
    InvalidHash,
    SerializationError(serde_json::Error),
    ApiError(String),
}

pub fn open_repository(path: &Path) -> Result<CogitRepository, CogitError> {
    if !path.join(".cogit").exists() {
        return Err(CogitError::NotARepository);
    }
    
    let config = fs::read_to_string(path.join(".cogit/config.json"))?;
    let repo = CogitRepository::from_config(&config)?;
    Ok(repo)
}
```

**Conceitos aplicados:**
- **Explicit error handling**: Erros são parte do tipo
- **Composabilidade**: `?` operator propaga erros automaticamente
- **Exhaustive matching**: Compilador garante tratamento de todos os erros

## 🔄 Programação Funcional

### Map, Filter e Iteradores

```rust
let relevant_commits: Vec<(String, f32)> = embeddings
    .iter()
    .map(|(commit_hash, embedding)| {
        let similarity = cosine_similarity(&query_embedding, embedding);
        (commit_hash.clone(), similarity)
    })
    .filter(|(_, similarity)| *similarity > 0.7)
    .collect();
```

### Closures e Higher-Order Functions

```rust
pub fn find_similar_commits<F>(&self, predicate: F) -> Vec<String>
where
    F: Fn(&FileEmbedding) -> bool,
{
    self.embeddings
        .iter()
        .filter(|embedding| predicate(embedding))
        .map(|embedding| embedding.commit_hash.clone())
        .collect()
}

// Uso
let rust_commits = repo.find_similar_commits(|embed| {
    embed.file_path.ends_with(".rs")
});
```

### Imutabilidade por Padrão

```rust
// Imutável por padrão
let commit_hash = calculate_hash(&content);

// Mutabilidade explícita quando necessária
let mut embeddings = Vec::new();
embeddings.push(new_embedding);
```

## 🔀 Pattern Matching

### Match Expressions

```rust
match file_status {
    WorkingTreeStatus::Untracked => {
        println!("  {}: arquivo não rastreado", path);
    }
    WorkingTreeStatus::Modified => {
        println!("  {}: modificado", path);
        generate_diff_and_embedding(path).await?;
    }
    WorkingTreeStatus::Staged => {
        println!("  {}: no staging area", path);
    }
    WorkingTreeStatus::Deleted => {
        println!("  {}: deletado", path);
        remove_from_embeddings(path)?;
    }
}
```

### If Let para Casos Específicos

```rust
// Ao invés de match completo
if let Some(parent_hash) = commit.parent {
    let parent_commit = self.load_commit(&parent_hash)?;
    let diff = calculate_diff(&parent_commit, &commit)?;
    return Ok(diff);
}
```

## ⚡ Programação Assíncrona

### Async/Await

```rust
pub async fn ask_question(&self, question: &str) -> Result<String, CogitError> {
    // 1. Gerar embedding da pergunta
    let question_embedding = self.generate_embedding(question).await?;
    
    // 2. Buscar commits similares
    let similar_commits = self.find_similar_commits(&question_embedding)?;
    
    // 3. Construir contexto
    let context = self.build_context(&similar_commits).await?;
    
    // 4. Chamar ChatGPT
    let response = self.call_chat_completion(&context, question).await?;
    
    Ok(response)
}
```

### Concorrência Segura

```rust
use tokio::join;

pub async fn process_multiple_files(&self, files: Vec<PathBuf>) 
    -> Result<Vec<FileEmbedding>, CogitError> {
    
    let futures: Vec<_> = files
        .into_iter()
        .map(|file| self.process_single_file(file))
        .collect();
    
    // Processa todos os arquivos em paralelo
    let results = join_all(futures).await;
    
    // Coleta apenas sucessos
    results.into_iter().collect::<Result<Vec<_>, _>>()
}
```

## 🧬 Metaprogramação e Derive Macros

### Serialização Automática

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub hash: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub parent: Option<String>,
    pub tree_hash: String,
}

// Uso automático
let commit_json = serde_json::to_string(&commit)?;
let commit: Commit = serde_json::from_str(&commit_json)?;
```

### Custom Derives para CLI

```rust
#[derive(Parser)]
#[command(name = "cogit")]
#[command(about = "Sistema de controle de versão inteligente")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    Commit {
        #[arg(short, long)]
        message: String,
        #[arg(long)]
        skip_ai: bool,
    },
}
```

## 🔗 Traits e Polimorfismo

### Definindo Comportamentos Comuns

```rust
pub trait Hashable {
    fn calculate_hash(&self) -> String;
}

impl Hashable for Commit {
    fn calculate_hash(&self) -> String {
        let content = format!("{}{}{}", self.message, self.timestamp, self.tree_hash);
        sha256(&content)
    }
}

impl Hashable for TreeEntry {
    fn calculate_hash(&self) -> String {
        let content = format!("{}{}{}", self.name, self.hash, self.is_file);
        sha256(&content)
    }
}
```

### Trait Objects para Flexibilidade

```rust
pub trait EmbeddingStore {
    async fn store(&self, commit_hash: &str, embeddings: Vec<FileEmbedding>) -> Result<(), CogitError>;
    async fn retrieve(&self, commit_hash: &str) -> Result<Vec<FileEmbedding>, CogitError>;
}

pub struct FileSystemStore;
pub struct CloudStore;

impl EmbeddingStore for FileSystemStore { /* ... */ }
impl EmbeddingStore for CloudStore { /* ... */ }

// Uso polimórfico
pub struct EmbeddingEngine {
    store: Box<dyn EmbeddingStore>,
}
```

## 🏗️ Ownership e Borrowing

### Movimentação de Ownership

```rust
pub fn commit(mut self, message: String) -> Result<(CogitRepository, String), CogitError> {
    let commit_hash = self.create_commit_object(&message)?;
    self.update_head(&commit_hash)?;
    
    // Move ownership de volta para o caller
    Ok((self, commit_hash))
}
```

### Borrowing para Eficiência

```rust
// Não precisa mover o ownership, apenas empresta
pub fn calculate_diff(&self, old_content: &str, new_content: &str) -> DiffResult {
    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();
    
    // Processa sem copiar dados desnecessariamente
    self.diff_algorithm(&old_lines, &new_lines)
}
```

### Lifetimes Explícitos

```rust
pub struct CommitIterator<'a> {
    repo: &'a CogitRepository,
    current: Option<String>,
}

impl<'a> Iterator for CommitIterator<'a> {
    type Item = Result<Commit, CogitError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Iterator empresta repo sem mover ownership
    }
}
```

## 📊 Avaliação dos Conceitos

| Conceito | Implementação no COGIT | Benefício |
|----------|------------------------|-----------|
| **Type Safety** | Enums, structs, generics | Zero erros de tipo em runtime |
| **Memory Safety** | Ownership, borrowing | Zero memory leaks ou segfaults |
| **Error Handling** | Result<T, E> | Tratamento explícito de todos os erros |
| **Concorrência** | async/await, Send/Sync | Chamadas de API seguras |
| **Polimorfismo** | Traits | Código flexível e testável |
| **Metaprogramação** | Derive macros | Menos boilerplate |
| **Programação Funcional** | Iteradores, closures | Código mais expressivo |

## 🏆 Resultado Final

O COGIT demonstra como Rust permite aplicar conceitos avançados de linguagens de programação de forma prática:

- **Zero runtime errors** relacionados a tipos ou memória
- **Código expressivo** através de programação funcional
- **Concorrência segura** para operações de IA
- **Tratamento robusto de erros** em todas as operações
- **Polimorfismo** através de traits ao invés de herança
- **Performance** através de zero-cost abstractions 