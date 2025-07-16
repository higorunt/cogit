# Exemplo Prático - COGIT com IA

Este exemplo demonstra o uso completo do COGIT com análise semântica de código usando inteligência artificial.

## Pré-requisitos

1. **Compilar o COGIT:**
```bash
cd ~/dev/cogit
cargo build --release
```

2. **Configurar OpenAI API Key:**
```bash
export OPENAI_API_KEY=sua_chave_aqui
```

## Passo a Passo

### 1. Criar Diretório de Teste
```bash
mkdir ~/cogit-demo
cd ~/cogit-demo
```

### 2. Inicializar Repositório COGIT
```bash
~/dev/cogit/target/release/cogit init
```

**Saída esperada:**
```
Repositório COGIT inicializado em: .
```

### 3. Criar Arquivos de Código de Exemplo

**Arquivo Python (main.py):**
```bash
echo 'def hello():
    print("Hello from Python!")

hello()' > main.py
```

**Arquivo Rust (main.rs):**
```bash
echo 'fn main() {
    println!("Hello from Rust!");
}' > main.rs
```

**Arquivo JavaScript (main.js):**
```bash
echo 'function hello() {
    console.log("Hello from JavaScript!");
}

hello();' > main.js
```

### 4. Fazer Commit com Análise IA
```bash
~/dev/cogit/target/release/cogit commit --message "Adiciona arquivos de exemplo em múltiplas linguagens"
```

**Saída esperada:**
```
Commit criado: 1cec02686e430afa386de34c39d17bff2242709384df2a5b9eda1275e1f47bc9
Iniciando análise semântica...
Processando: ./main.py
Processando: ./main.rs
Processando: ./main.js
Análise concluída: 3 arquivo(s) processado(s)
Tempo: 3464ms | Tokens: 3000
```

### 5. Visualizar Histórico
```bash
~/dev/cogit/target/release/cogit log
```

**Saída esperada:**
```
 - Adiciona arquivos de exemplo em múltiplas linguagens
   2025-07-16 03:01:49.015866432 UTC
```

### 6. Ver Índice de IA
```bash
~/dev/cogit/target/release/cogit index
```

**Saída esperada:**
```
Commits com Análise IA (1):

1cec02686e430afa386de34c39d17bff2242709384df2a5b9eda1275e1f47bc9 (3 arquivo(s))
   2025-07-16 03:01:52
   3000 tokens | 3464ms

Use 'cogit explain <hash>' para ver detalhes de um commit
```

### 7. Explicar Commit Específico
```bash
~/dev/cogit/target/release/cogit explain 1cec02686e430afa386de34c39d17bff2242709384df2a5b9eda1275e1f47bc9
```

**Saída esperada:**
```
Análise do Commit: 1cec02686e430afa386de34c39d17bff2242709384df2a5b9eda1275e1f47bc9
Criado em: 2025-07-16 03:01:52.532671323 UTC
Arquivos analisados: 3
Tokens processados: 3000
Tempo de processamento: 3464ms

./main.py
   Tamanho: 54 bytes
   Hash: 59e3f4e7
   Vetor: 1536 dimensões

./main.rs
   Tamanho: 48 bytes
   Hash: 8d3b20cf
   Vetor: 1536 dimensões

./main.js
   Tamanho: 74 bytes
   Hash: fe60a046
   Vetor: 1536 dimensões

Funcionalidade completa de explicação IA em desenvolvimento...
```

## Recursos Demonstrados

### ✅ Funcionalidades Testadas
- **Inicialização**: Criação de repositório `.cogit`
- **Commit Tradicional**: Armazenamento content-addressable
- **Análise IA**: Processamento automático via OpenAI text-embedding-3-small
- **Filtros Inteligentes**: Processou apenas arquivos de código (ignorou documentação)
- **Índice Vetorial**: Armazenamento de embeddings em `.cogit/index/`
- **Comandos IA**: `cogit index` e `cogit explain`

### 📊 Métricas de Performance
- **Arquivos processados**: 3 (Python, Rust, JavaScript)
- **Tokens utilizados**: 3000
- **Tempo de processamento**: 3.464 segundos
- **Dimensões do vetor**: 1536 (padrão text-embedding-3-small)

### 🗂️ Estrutura Criada
```
cogit-demo/
├── .cogit/
│   ├── objects/      # Commits e árvores
│   ├── index/        # Embeddings IA (.json)
│   ├── refs/heads/   # Referências
│   └── config.json   # Configuração
├── main.py          # Código Python
├── main.rs          # Código Rust
└── main.js          # Código JavaScript
```

## Comandos Avançados

### Commit sem IA
```bash
~/dev/cogit/target/release/cogit commit --skip-ai -m "Commit rápido sem análise"
```

### Status do Repositório
```bash
~/dev/cogit/target/release/cogit status
```

## Casos de Uso Acadêmicos

Este exemplo demonstra como o COGIT pode ser usado para:

1. **Análise de Evolução de Código**: Tracking semântico de mudanças
2. **Comparação de Linguagens**: Embeddings permitem análise cross-language
3. **Busca Contextual**: Futuras queries semânticas sobre histórico
4. **Métricas de Complexidade**: Análise temporal do crescimento do projeto

## Próximos Passos

Para expandir este exemplo:
- Adicionar mais arquivos e fazer commits incrementais
- Testar diferentes tipos de código (classes, funções, algoritmos)
- Explorar padrões nos embeddings gerados
- Implementar queries semânticas avançadas 