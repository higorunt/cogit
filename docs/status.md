# Status do Projeto COGIT - Seminário

## Configuração da Equipe

**Membros do Grupo:**
- Higor Roger de Freitas Santos - Matrícula: 221006440
- Evelyn Caroline Morais Targino - Matrícula: 221006404

**Divisão de Responsabilidades:**
- Higor: Arquitetura do sistema, implementação do core em Rust, sistema de hashing
- Evelyn: Interface de linha de comando, testes, documentação

## Status Atual do Desenvolvimento

### Funcionalidades Concluídas
- Configuração inicial do projeto Rust
- Estrutura básica do repositório
- Definição da arquitetura do sistema
- Documentação inicial do projeto
- Implementação do comando `cogit init`
- Sistema de hashing SHA-256 para content-addressable storage
- Estrutura básica de commits
- Parser de árvore de diretórios
- Interface de linha de comando (CLI)
- Sistema de tratamento de erros
- Implementação do comando `cogit commit`
- Comando `cogit log` para visualizar histórico
- Comando `cogit status` para status do repositório

### Próximos Passos
- Testes unitários e integração
- Melhorias na interface de usuário
- Suporte a branches
- Otimizações de performance
- Documentação técnica avançada

## Tecnologias Utilizadas

- **Linguagem**: Rust 2021 Edition
- **Gerenciador de Pacotes**: Cargo
- **Dependências**: clap, sha2, serde, serde_json, chrono
- **Controle de Versão**: Git (para desenvolvimento do próprio COGIT)

## Objetivos do MVP

O COGIT será apresentado como uma demonstração prática de como Rust pode ser usado para reimplementar sistemas críticos como o Git, oferecendo:

1. **Segurança de Memória**: Eliminação de bugs comuns do C
2. **Performance**: Velocidade comparável ao Git original
3. **Ergonomia**: Código mais limpo e manutenível
4. **Funcionalidades Básicas**:
   - `cogit init` - Inicializar repositório
   - `cogit commit` - Criar snapshots
   - `cogit log` - Visualizar histórico
   - `cogit status` - Verificar status

## Visão de Longo Prazo

Após o seminário, o projeto poderá evoluir para incluir:
- Integração com ferramentas de IA (Copilot, ChatGPT)
- Versionamento de decisões assistidas por IA
- Sistema de replay de raciocínio de IA
- Integração complementar com Git/GitHub

---

*Projeto COGIT - Cognition Git*  
*Universidade de Brasília* 