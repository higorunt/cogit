# Correção do Problema do Comando `cogit ask`

## 🐛 Problema Identificado

O usuário relatou que o comando `cogit ask -q "pergunta"` estava retornando:

```
Não encontrei informações relevantes para responder sua pergunta. Certifique-se de que existem commits com análise IA.
```

Mesmo tendo commits com análise IA confirmados pelo comando `cogit index`.

## 🔍 Diagnóstico

Através da análise do código, identifiquei que o problema estava na função `ask_question()` em `src/embedding.rs`. 

### Fluxo do Erro:

1. **Comando `ask` chamado** → `main.rs:388-410`
2. **Verifica `OPENAI_API_KEY`** → Se não existir, mostra mensagem de erro ✅
3. **Se existir, chama `ask_question()`** → `embedding.rs:515`
4. **`ask_question()` chama `find_relevant_embeddings()`** → `embedding.rs:519`
5. **`find_relevant_embeddings()` chama `call_openai_embedding()`** → `embedding.rs:398`
6. **`call_openai_embedding()` falha** → Retorna erro se chave vazia
7. **Erro é propagado com `?`** → Volta para `ask_question()`
8. **`ask_question()` falha silenciosamente** → Retorna mensagem genérica ❌

### Problema Específico:

O erro real (chave da API não configurada) estava sendo **mascarado** pela mensagem genérica "Não encontrei informações relevantes".

## ✅ Solução Implementada

Modifiquei a função `ask_question()` para **capturar especificamente** o erro de chave da API:

```rust
// ANTES:
let relevant_embeddings = self.find_relevant_embeddings(question, commit_filter).await?;

// DEPOIS:
let relevant_embeddings = match self.find_relevant_embeddings(question, commit_filter).await {
    Ok(embeddings) => embeddings,
    Err(e) => {
        // Verificar se é erro de chave da API
        if let CogitError::IoError(io_err) = &e {
            if io_err.kind() == std::io::ErrorKind::InvalidInput && 
               io_err.to_string().contains("Chave da API OpenAI não configurada") {
                return Ok("❌ Erro: Chave da API OpenAI não configurada.\nPara usar o comando 'ask', defina: export OPENAI_API_KEY=sua_chave".to_string());
            }
        }
        return Err(e);
    }
};
```

## 🚀 Resultado

### Antes da Correção:
```bash
$ cogit ask -q "Sobre o que se trata o nosso repositório?"
🔍 Buscando informações relevantes...
Resposta:
Não encontrei informações relevantes para responder sua pergunta. Certifique-se de que existem commits com análise IA.
```

### Depois da Correção:
```bash
$ cogit ask -q "Sobre o que se trata o nosso repositório?"
🔍 Buscando informações relevantes...
Resposta:
❌ Erro: Chave da API OpenAI não configurada.
Para usar o comando 'ask', defina: export OPENAI_API_KEY=sua_chave
```

## 📝 Como Testar a Correção

1. **Compile o projeto:**
   ```bash
   cargo build --release
   ```

2. **Teste sem chave da API:**
   ```bash
   unset OPENAI_API_KEY
   ./target/release/cogit ask -q "teste"
   ```
   
   **Resultado esperado:** Mensagem clara sobre chave da API

3. **Teste com chave da API:**
   ```bash
   export OPENAI_API_KEY=sua_chave
   ./target/release/cogit ask -q "teste"
   ```
   
   **Resultado esperado:** Busca funcional

## 🎯 Impacto da Correção

- ✅ **Diagnóstico claro** do problema real
- ✅ **Experiência do usuário melhorada** 
- ✅ **Debugging mais fácil**
- ✅ **Mensagens de erro específicas**

A correção garante que o usuário saiba exatamente o que fazer quando o comando `ask` falha, em vez de receber uma mensagem confusa. 