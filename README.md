# 🦀 Aprendendo Rust — Jornada pelo The Rust Book

> Repositório de estudos acompanhando o [The Rust Programming Language Book](https://doc.rust-lang.org/book/) (também conhecido como "The Book"), com implementações práticas de cada capítulo.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Status](https://img.shields.io/badge/Status-Em_Andamento-yellow?style=for-the-badge)
![Progress](https://img.shields.io/badge/Progresso-Capítulo_3-blue?style=for-the-badge)

---

## 🎯 Sobre Este Repositório

Documentação completa da minha jornada de aprendizado em **Rust**, seguindo o livro oficial **The Rust Programming Language**. Cada capítulo é implementado em uma pasta separada com código funcional e anotações de aprendizado.

**Por que Rust?**  
Rust combina **performance de baixo nível** (como C/C++) com **segurança de memória** garantida em tempo de compilação — habilidades valiosas para infraestrutura, sistemas embarcados e ferramentas de DevOps modernas.

---

## 📚 Estrutura do Repositório

```
learn-Rust/
├── 01-hello-cargo/         # Cap. 1 - Hello World com Cargo
├── 02-guessing-game/       # Cap. 2 - Jogo de Adivinhação
├── 03-variables/           # Cap. 3 - Variáveis e Mutabilidade
├── 04-ownership/           # Cap. 4 - Ownership (em breve)
├── 05-structs/             # Cap. 5 - Structs (planejado)
└── README.md
```

Cada pasta contém:
- `src/main.rs` — código fonte do projeto
- `Cargo.toml` — arquivo de configuração do Cargo
- `README.md` — anotações e aprendizados do capítulo

---

## 🗂️ Capítulos Implementados

### ✅ [01 - Hello Cargo](./01-hello-cargo/)
**Conceitos:** Cargo, compilação, estrutura de projetos Rust

Primeiro contato com o ecossistema Rust, entendendo a ferramenta **Cargo** (gerenciador de pacotes e build).

```bash
cd 01-hello-cargo
cargo run
```

---

### ✅ [02 - Guessing Game](./02-guessing-game/)
**Conceitos:** Input/Output, Match, Loops, Crates externos (rand)

Jogo interativo onde o programa gera um número aleatório e o usuário tenta adivinhar.

**Aprendizados principais:**
- Uso de `std::io` para entrada de dados
- Pattern matching com `match`
- Importação de crates externos (`rand`)
- Loops com `loop` e controle de fluxo

```bash
cd 02-guessing-game
cargo run
```

---

### ✅ [03 - Variables](./03-variables/)
**Conceitos:** Mutabilidade, Shadowing, Tipos de dados, Constantes

Exploração do sistema de tipos e gerenciamento de memória do Rust.

**Aprendizados principais:**
- Diferença entre `let` e `let mut`
- Shadowing vs. Mutabilidade
- Tipos escalares (inteiros, floats, booleanos, caracteres)
- Tipos compostos (tuplas, arrays)

```bash
cd 03-variables
cargo run
```

---

## 🔮 Próximos Capítulos (Planejados)

- [ ] **Cap. 4** — Ownership (conceito central do Rust)
- [ ] **Cap. 5** — Structs e Métodos
- [ ] **Cap. 6** — Enums e Pattern Matching
- [ ] **Cap. 7** — Módulos e Pacotes
- [ ] **Cap. 8** — Coleções (Vectors, Strings, HashMaps)
- [ ] **Cap. 9** — Error Handling
- [ ] **Cap. 10** — Generics, Traits, Lifetimes

---

## 🚀 Como Executar os Projetos

### Pré-requisitos
- [Rust](https://www.rust-lang.org/tools/install) instalado (rustc + cargo)

### Executando um capítulo específico

```bash
# Clone o repositório
git clone https://github.com/rapassos/learn-Rust.git
cd learn-Rust

# Navegue até o capítulo desejado
cd 02-guessing-game

# Compile e execute
cargo run

# Ou apenas compile
cargo build

# Executável estará em:
./target/debug/[nome-do-projeto]
```

---

## 📖 Recursos de Aprendizado

- 📘 [The Rust Programming Language Book](https://doc.rust-lang.org/book/) — Livro oficial (gratuito)
- 🎓 [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — Aprendizado prático
- 📝 [Rustlings](https://github.com/rust-lang/rustlings/) — Exercícios interativos
- 🦀 [Rust Playground](https://play.rust-lang.org/) — Testar código online
- 📚 [Docs.rs](https://docs.rs/) — Documentação de crates

---

## 💡 Por Que Rust Para Infraestrutura?

Como profissional de infraestrutura com 15+ anos de experiência, Rust oferece:

- ⚡ **Performance** — Velocidade comparável a C/C++, ideal para ferramentas CLI
- 🔒 **Segurança** — Zero null pointers, zero data races (garantido em compilação)
- 🛠️ **Tooling moderno** — Cargo, rustfmt, clippy são excelentes
- 🌐 **Cross-compilation** — Binários para Linux, Windows, macOS com o mesmo código
- 📦 **Deploy simples** — Binário único, sem dependências de runtime

**Casos de uso:**
- Ferramentas de automação (substituto de scripts Bash/Python para performance crítica)
- Agents de monitoramento
- Proxies e load balancers customizados
- Parsers de logs de alta performance

---

## 🎯 Objetivos de Aprendizado

- [x] Sintaxe básica e tooling do Rust
- [x] Sistema de tipos e mutabilidade
- [ ] **Ownership** (próximo marco importante!)
- [ ] Pattern matching avançado
- [ ] Traits e generics
- [ ] Async/await para I/O
- [ ] Criar ferramenta CLI real com Rust

---

## 📊 Progresso

```
████████░░░░░░░░░░░░░░░░░░░░ 30% — Capítulo 3 de 10 (conceitos fundamentais)
```

**Última atualização:** Fevereiro 2025

---

## 🔗 Repositórios Relacionados

Este repositório consolida o que antes eram projetos separados:
- ~~learn-Rust-hello_cargo~~ → agora em `01-hello-cargo/`
- ~~learn-Rust-guessing_game~~ → agora em `02-guessing-game/`
- ~~learn-Rust-variables~~ → agora em `03-variables/`

---

## 👤 Autor

**Rafael Passos Guimarães**

Analista de Infraestrutura | 15+ anos em TI | Estudando Rust para ferramentas de DevOps

- 💼 LinkedIn: [@rapassos](https://linkedin.com/in/rapassos)
- 🐙 GitHub: [@rapassos](https://github.com/rapassos)
- 📧 Email: rapassos@gmail.com

---

## 📄 Licença

MIT License — veja [LICENSE](LICENSE) para detalhes.

---

> 💭 **Reflexão:** Rust tem uma curva de aprendizado íngreme, mas o borrow checker força a escrever código seguro desde o início. Cada erro de compilação é uma lição sobre gerenciamento de memória — algo que scripts em linguagens de alto nível escondem, mas que profissionais de infraestrutura precisam entender profundamente.
