# Wiki do Projeto COGIT: Cognition Git

## 📚 Disciplina

**Linguagens de Programação** – 2025/1  
**Professor:** Marcelo Ladeira

## 👥 Equipe

**Higor Roger de Freitas Santos** – 221006440  
**Evelyn Caroline Morais Targino** – 221006404

---

## 🎯 O que é o COGIT?

O COGIT é um sistema de controle de versão inteligente que vai além do Git tradicional. Ele combina versionamento clássico com análise semântica via inteligência artificial. Seu diferencial é permitir que o usuário compreenda o impacto das mudanças feitas no código, utilizando representações vetoriais (embeddings) e modelos de linguagem natural da OpenAI.

**Principal inovação:** Ao invés de processar arquivos inteiros, o COGIT analisa apenas os **diffs** (mudanças), economizando 90% dos tokens da IA e tornando o processo muito mais rápido.

## 📖 Páginas da Wiki

- **[Visão Geral](Visao-Geral)** - Detalhes técnicos do projeto
- **[Por que Rust?](Por-que-Rust)** - Motivação para escolha da linguagem
- **[Conceitos Aplicados](Conceitos-Aplicados)** - Aspectos de linguagens de programação
- **[Sistema de IA](Sistema-IA)** - Como funciona a integração com embeddings
- **[Comandos](Comandos)** - Lista dos comandos disponíveis

## 🚀 Exemplo Rápido

```bash
# Inicializar repositório
cogit init

# Adicionar arquivos
cogit add .

# Fazer commit com análise IA
cogit commit -m "Implementar autenticação"

# Perguntar sobre o código
cogit ask -q "Como funciona a autenticação?"
```

## 📊 Comparação com Git

| Aspecto | Git | COGIT |
|---------|-----|-------|
| Hash | SHA-1 | SHA-256 |
| Linguagem | C | Rust |
| Dados | Binário | JSON |
| IA | Não | Embeddings automáticos |
| Busca | grep/log | Semântica |

---

**Projeto desenvolvido na Universidade de Brasília para demonstrar conceitos avançados de linguagens de programação aplicados em uma ferramenta real.** 