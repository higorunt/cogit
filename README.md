# COGIT - Cognition Git

Um sistema de controle de versão desenvolvido em Rust como projeto educacional, demonstrando como reimplementar funcionalidades básicas do Git com maior segurança de memória e performance.

## Funcionalidades

- `cogit init` - Inicializar repositório
- `cogit commit -m "mensagem"` - Criar commits
- `cogit log` - Visualizar histórico
- `cogit status` - Status do repositório
- Sistema de hashing SHA-256
- Armazenamento content-addressable

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

O COGIT implementa conceitos fundamentais do Git:

- **Content-addressable storage**: Objetos identificados por hash SHA-256
- **Commits**: Snapshots do estado do projeto  
- **Tree objects**: Representação da estrutura de arquivos
- **Referências**: Sistema de ponteiros para commits

### Diferenças do Git

| Aspecto | Git | COGIT |
|---------|-----|-------|
| Hash | SHA-1 | SHA-256 |
| Linguagem | C | Rust |
| Serialização | Binário | JSON |
| Interface | Inglês | Português |

## Desenvolvimento

Para contribuir ou entender melhor o projeto, consulte:
- [GUIA_DESENVOLVIMENTO.md](GUIA_DESENVOLVIMENTO.md) - Guia completo de desenvolvimento
- [STATUS_SEMINARIO.md](STATUS_SEMINARIO.md) - Status atual do projeto

## Equipe

- Higor Roger de Freitas Santos - 221006440
- Evelyn Caroline Morais Targino - 221006404

## Licença

Projeto educacional - Universidade de Brasília
