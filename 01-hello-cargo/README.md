# 📦 Capítulo 1 — Hello, Cargo!

> Primeiro contato com o ecossistema Rust e a ferramenta Cargo.

## 🎯 Objetivos do Capítulo

- Entender o que é **Cargo** e por que é usado
- Criar, compilar e executar um projeto Rust
- Compreender a estrutura de um projeto Cargo

---

## 🛠️ O que é Cargo?

**Cargo** é o sistema de build e gerenciador de pacotes do Rust. Ele:

- 📦 Gerencia dependências (crates)
- 🔨 Compila projetos
- 🧪 Executa testes
- 📝 Gera documentação
- 🚀 Publica pacotes no [crates.io](https://crates.io)

---

## 📂 Estrutura do Projeto

```
01-hello-cargo/
├── Cargo.toml       # Manifesto do projeto (metadados + dependências)
├── src/
│   └── main.rs      # Código fonte principal
└── target/          # Pasta de build (gerada automaticamente)
```

### Cargo.toml
```toml
[package]
name = "hello-cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
# Dependências externas vão aqui
```

---

## 🚀 Comandos Aprendidos

```bash
# Criar novo projeto
cargo new hello-cargo

# Compilar o projeto
cargo build

# Compilar e executar
cargo run

# Compilar otimizado para produção
cargo build --release

# Verificar se compila (sem gerar executável)
cargo check
```

---

## 💡 Aprendizados Principais

1. **`cargo new`** cria estrutura completa com Git inicializado
2. **`cargo run`** compila E executa — mais eficiente durante desenvolvimento
3. **`cargo check`** é mais rápido que `build` — útil para validar código
4. Arquivos `.lock` garantem builds reproduzíveis (commitá-los!)
5. Pasta `target/` deve estar no `.gitignore` (já vem por padrão)

---

## 🔗 Referências

- [The Book - Cap. 1.3](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

---

**Próximo:** [02 - Guessing Game](../02-guessing-game/) →
