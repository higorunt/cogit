# COGIT - Cognition Git

Um sistema de controle de versão inteligente desenvolvido em Rust que combina funcionalidades tradicionais de versionamento com inteligência contextual por IA. O COGIT não apenas versiona código, mas também compreende semanticamente as mudanças através de embeddings vetoriais.

## Funcionalidades

### Core (Implementado)
- `cogit init` - Inicializar repositório
- `cogit commit -m "mensagem"` - Criar commits com análise semântica
- `cogit log` - Visualizar histórico
- `cogit status` - Status do repositório
- Sistema de hashing SHA-256
- Armazenamento content-addressable

### Inteligência Contextual (Em Desenvolvimento)
- **Embeddings Automáticos**: Cada commit gera embeddings vetoriais dos arquivos modificados
- **Índice Semântico**: Armazenamento em `.cogit/index/<hash>.json` para consulta IA
- `cogit explain <commit>` - Explicação inteligente de commits
- `cogit index` - Listagem de commits com embeddings
- **API OpenAI**: Integração com text-embedding-3-small

## Instalação e Uso

### Pré-requisitos

- Rust 1.70+ 
- Build tools (Linux/WSL: `sudo apt install build-essential`)

### Compilação

```bash
git clone <repositorio>
cd cogit
cargo build --release
```

### Uso Básico

```bash
# Inicializar repositório
./target/release/cogit init

# Criar arquivos e fazer commit
echo "Hello COGIT!" > arquivo.txt
./target/release/cogit commit -m "Primeiro commit"

# Ver histórico
./target/release/cogit log

# Verificar status
./target/release/cogit status
```

## Arquitetura

O COGIT combina conceitos fundamentais do Git com inteligência artificial:

### Sistema de Versionamento
- **Content-addressable storage**: Objetos identificados por hash SHA-256
- **Commits**: Snapshots do estado do projeto  
- **Tree objects**: Representação da estrutura de arquivos
- **Referências**: Sistema de ponteiros para commits

### Inteligência Contextual
- **Embeddings Vetoriais**: Cada commit gera representações semânticas via OpenAI
- **Índice Inteligente**: Mapeamento `.cogit/index/` para consultas IA
- **Análise Semântica**: Compreensão do que mudou, não apenas onde mudou
- **Preparação IA**: Base de conhecimento para agentes inteligentes futuros

### Diferenças do Git

| Aspecto | Git | COGIT |
|---------|-----|-------|
| Hash | SHA-1 | SHA-256 |
| Linguagem | C | Rust |
| Serialização | Binário | JSON |
| Interface | Inglês | Português |
| **Inteligência** | **Manual** | **IA-Powered** |
| **Semântica** | **Não** | **Embeddings Automáticos** |
| **Consulta** | **Grep/Log** | **Busca Vetorial** |

## Visão e Missão

**COGIT** está evoluindo de uma alternativa educacional do Git para um sistema de controle de versão orientado a IA:

🎯 **Missão**: Criar o primeiro sistema de versionamento que compreende semanticamente o código
🚀 **Visão**: Habilitar desenvolvimento colaborativo entre humanos e IA através de repositórios inteligentes
🧠 **Diferencial**: Não apenas versionar arquivos, mas versionar significado e contexto

## Desenvolvimento

Para contribuir ou entender melhor o projeto, consulte:
- [GUIA_DESENVOLVIMENTO.md](GUIA_DESENVOLVIMENTO.md) - Guia completo de desenvolvimento

## Equipe

- Higor Roger de Freitas Santos - 221006440
- Evelyn Caroline Morais Targino - 221006404

## Licença

Projeto educacional - Universidade de Brasília, Faculdade do Gama 