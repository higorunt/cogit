# Comandos do COGIT

## 📋 Lista de Comandos

| Comando | Função | Exemplo |
|---------|--------|---------|
| `cogit init` | Inicializa repositório | `cogit init` |
| `cogit add` | Adiciona arquivos à staging area | `cogit add .` |
| `cogit commit` | Cria commit com ou sem análise por IA | `cogit commit -m "mensagem"` |
| `cogit status` | Mostra status do repositório | `cogit status` |
| `cogit diff` | Mostra mudanças entre versões | `cogit diff --file main.rs` |
| `cogit log` | Exibe histórico de commits | `cogit log` |
| `cogit ask` | Responde perguntas sobre código usando IA | `cogit ask -q "Como funciona X?"` |

## 🔧 Comandos Detalhados

### `cogit init`
Inicializa um novo repositório COGIT no diretório atual ou especificado.

```bash
# Inicializar no diretório atual
cogit init

# Inicializar em diretório específico
cogit init meu-projeto
```

**O que faz:**
- Cria diretório `.cogit/`
- Configura estrutura de objects, refs, index
- Cria arquivo HEAD e config inicial

### `cogit add`
Adiciona arquivos modificados ao staging area.

```bash
# Adicionar todos os arquivos
cogit add .

# Adicionar arquivo específico
cogit add main.rs

# Adicionar múltiplos arquivos
cogit add src/ README.md
```

**O que faz:**
- Calcula hash SHA-256 dos arquivos
- Adiciona entradas ao `index.json`
- Prepara arquivos para próximo commit

### `cogit commit`
Cria um commit com os arquivos no staging area.

```bash
# Commit normal com IA
cogit commit -m "Implementar autenticação de usuários"

# Commit sem análise IA (mais rápido)
cogit commit -m "Fix typo" --skip-ai
```

**O que faz:**
- Cria objeto commit com hash único
- Gera embeddings IA dos diffs (se habilitado)
- Atualiza referência HEAD
- Limpa staging area

### `cogit status`
Mostra informações sobre o estado atual do repositório.

```bash
cogit status
```

**Saída típica:**
```
📊 Status: Repositório COGIT com 3 commit(s)

🟢 Mudanças no staging area:
  adicionado: nova_feature.rs

🟡 Mudanças não staged:
  modificado: main.rs

🔴 Arquivos não rastreados:
  temp.log

🤖 Commits com análise IA: 2/3
```

### `cogit diff`
Mostra diferenças entre versões de arquivos.

```bash
# Ver todas as mudanças não staged
cogit diff

# Ver mudanças de arquivo específico
cogit diff --file main.rs

# Ver mudanças no staging area
cogit diff --staged
```

**Formato de saída:**
```diff
diff --git a/main.rs b/main.rs
index 0000000..abc1234
--- a/main.rs
+++ b/main.rs
@@ -1,3 +1,4 @@
 fn main() {
     println!("Hello COGIT!");
+    println!("Nova linha!");
 }
```

### `cogit log`
Exibe histórico de commits em ordem cronológica reversa.

```bash
cogit log
```

**Saída típica:**
```
🔹 a1b2c3d4e5f6... - Adicionar sistema de login
   📅 2024-01-15T10:30:00Z
   🧠 IA: ✅ Análise semântica disponível

🔹 b2c3d4e5f6a7... - Estrutura inicial do projeto
   📅 2024-01-15T09:15:00Z
   🧠 IA: ❌ Sem análise (--skip-ai usado)
```

### `cogit ask`
Faz consultas semânticas inteligentes sobre o código.

```bash
# Buscar funcionalidade
cogit ask -q "Como funciona a autenticação?"

# Investigar bugs
cogit ask -q "Quando o problema de performance foi introduzido?"

# Entender arquitetura
cogit ask -q "Qual é a estrutura do projeto?"
```

**Exemplo de resposta:**
```
🔍 Buscando semanticamente: "Como funciona a autenticação?"

📊 Resultados encontrados (2 commits relevantes):

🎯 Commit: a1b2c3d4e5f6... (Similaridade: 92%)
📝 "Implementar sistema de login JWT"

🧠 Resposta da IA:
A autenticação é implementada usando JWT tokens. O processo funciona assim:

1. Usuário faz login via POST /auth/login
2. Sistema valida credenciais
3. Gera token JWT com informações do usuário
4. Token é usado para acessar rotas protegidas
5. Middleware valida token em cada request

Arquivos principais: auth.rs, middleware.rs, tokens.rs
```

## ⚙️ Opções Globais

### Variáveis de Ambiente

```bash
# Obrigatória para funcionalidades de IA
export OPENAI_API_KEY="sua_chave_aqui"

# Opcionais
export OPENAI_MODEL="text-embedding-3-small"
export OPENAI_CHAT_MODEL="gpt-3.5-turbo"

# Debug
export RUST_LOG="debug"
```

### Flags Comuns

- `--help` - Mostra ajuda do comando
- `--version` - Mostra versão do COGIT
- `--skip-ai` - Pula análise IA (apenas em commit)

## 🚀 Workflow Típico

```bash
# 1. Inicializar projeto
cogit init

# 2. Desenvolver código
echo 'fn main() { println!("Hello"); }' > main.rs

# 3. Ver status
cogit status

# 4. Adicionar mudanças
cogit add .

# 5. Ver o que será commitado
cogit diff --staged

# 6. Fazer commit
cogit commit -m "Implementar hello world"

# 7. Ver histórico
cogit log

# 8. Fazer pergunta sobre o código
cogit ask -q "O que faz a função main?"
```

## 🔍 Troubleshooting

**Comando não encontrado:**
```bash
# Verificar se está no PATH
which cogit

# Ou usar caminho completo
./target/release/cogit
```

**Erro de repositório:**
```bash
# Verificar se está em repositório COGIT
ls -la .cogit/

# Inicializar se necessário
cogit init
```

**API da OpenAI falha:**
```bash
# Verificar chave
echo $OPENAI_API_KEY

# Usar modo sem IA
cogit commit -m "mensagem" --skip-ai
``` 