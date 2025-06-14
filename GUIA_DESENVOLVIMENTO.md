# Guia de Desenvolvimento - COGIT

## Sobre o Projeto

COGIT é um sistema de controle de versão desenvolvido em Rust como projeto educacional, demonstrando como reimplementar funcionalidades básicas do Git com maior segurança de memória e performance.

## Configuração do Ambiente

### Pré-requisitos

1. **Rust** (versão 1.70 ou superior)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Git** (para controle de versão do projeto)

3. **Build tools** (Linux/WSL)
   ```bash
   sudo apt update
   sudo apt install build-essential
   ```

### Clonando o Repositório

```bash
git clone <URL_DO_REPOSITORIO>
cd cogit
```

### Compilação

```bash
# Compilação para desenvolvimento
cargo build

# Compilação otimizada
cargo build --release
```

### Executando

```bash
# Versão de desenvolvimento
cargo run -- --help

# Versão otimizada
./target/release/cogit --help
```

## Funcionalidades Implementadas

### 1. Inicialização de Repositório
```bash
cogit init [diretório]
```
- Cria estrutura `.cogit/`
- Configura referências iniciais
- Gera arquivo de configuração

### 2. Criação de Commits
```bash
cogit commit -m "mensagem do commit"
```
- Cria snapshot do estado atual
- Gera hash SHA-256 único
- Armazena no sistema content-addressable

### 3. Visualização de Histórico
```bash
cogit log
```
- Lista commits em ordem cronológica reversa
- Mostra hash, mensagem e timestamp

### 4. Status do Repositório
```bash
cogit status
```
- Exibe informações básicas do repositório
- Conta total de commits

## Arquitetura do Sistema

### Estrutura de Diretórios
```
.cogit/
├── HEAD                 # Referência atual
├── config.json         # Configuração do repositório
├── objects/            # Armazenamento content-addressable
│   └── XX/             # Primeiros 2 chars do hash
│       └── XXXXXX...   # Resto do hash (objetos)
└── refs/
    └── heads/
        └── main        # Referência da branch principal
```

### Componentes Principais

1. **CogitRepository**: Estrutura principal do repositório
2. **Commit**: Representa um commit no sistema
3. **TreeEntry**: Entrada na árvore de arquivos
4. **CogitError**: Sistema de tratamento de erros

### Tecnologias Utilizadas

- **clap**: Interface de linha de comando
- **sha2**: Hashing SHA-256
- **serde**: Serialização/deserialização
- **serde_json**: Formato JSON para objetos
- **chrono**: Manipulação de timestamps

## Testando o Sistema

### Exemplo Prático

```bash
# 1. Criar diretório de teste
mkdir test-projeto
cd test-projeto

# 2. Inicializar repositório COGIT
cogit init

# 3. Criar alguns arquivos
echo "Olá COGIT!" > arquivo1.txt
echo "Sistema em Rust" > README.md

# 4. Fazer primeiro commit
cogit commit -m "Primeiro commit"

# 5. Adicionar mais conteúdo
echo "Mais funcionalidades" > arquivo2.txt

# 6. Fazer segundo commit
cogit commit -m "Segundo commit"

# 7. Ver histórico
cogit log

# 8. Verificar status
cogit status
```

### Saída Esperada

```
✅ Repositório COGIT inicializado em: .
✅ Commit criado: a1b2c3d4e5f6...
✅ Commit criado: b2c3d4e5f6a7...

🔹 b2c3d4e5f6a7... - Segundo commit
   📅 2024-01-15T10:35:00Z

🔹 a1b2c3d4e5f6... - Primeiro commit
   📅 2024-01-15T10:30:00Z

📊 Status: Repositório COGIT com 2 commit(s)
```

## Estrutura do Código

### src/main.rs
- Interface CLI usando clap
- Parsing de argumentos
- Chamadas para funcionalidades principais

### src/cogit.rs
- Implementação do core do sistema
- Estruturas de dados principais
- Lógica de armazenamento e recuperação

## Diferenças do Git

| Aspecto | Git | COGIT |
|---------|-----|-------|
| Hash | SHA-1 | SHA-256 |
| Linguagem | C | Rust |
| Serialização | Binário | JSON |
| Compressão | zlib | Nenhuma |
| Branches | Completo | Básico |
| Interface | Inglês | Português |

## Desenvolvimento Futuro

### Próximas Funcionalidades
- Sistema de staging area
- Suporte completo a branches
- Merge de branches
- Diff entre commits
- Compressão de objetos

### Melhorias Técnicas
- Testes unitários
- Benchmarks de performance
- Documentação de API
- Tratamento de conflitos

## Contribuindo

### Fluxo de Trabalho
1. Criar branch para nova funcionalidade
2. Implementar mudanças
3. Testar funcionalidade
4. Fazer commit seguindo convenções
5. Criar pull request

### Convenções de Commit
- Use mensagens descritivas
- Máximo 50 caracteres no título
- Use imperativo: "Adicionar" não "Adicionado"
- Inclua contexto quando necessário

## Troubleshooting

### Problemas Comuns

1. **Erro de compilação**: Verificar versão do Rust
2. **Linker não encontrado**: Instalar build-essential
3. **Permissões**: Verificar permissões de escrita

### Logs de Debug
```bash
RUST_LOG=debug cargo run -- comando
```

## Contato

- Higor Roger de Freitas Santos - 221006440
- Evelyn Caroline Morais Targino - 221006404

---

*Projeto COGIT - Cognition Git*  
*Universidade de Brasília - Faculdade do Gama* 