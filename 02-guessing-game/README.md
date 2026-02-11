# 🎲 Capítulo 2 — Jogo de Adivinhação

> Projeto interativo que consolida conceitos básicos de Rust: I/O, loops, match, e uso de crates externas.

## 🎮 Descrição do Jogo

O programa gera um número aleatório entre 1 e 100. O usuário tenta adivinhar, recebendo dicas se o palpite é muito alto ou muito baixo, até acertar.

---

## 🎯 Conceitos Aprendidos

### 1. **Input/Output com `std::io`**
```rust
use std::io;

let mut guess = String::new();
io::stdin()
    .read_line(&mut guess)
    .expect("Falha ao ler linha");
```

### 2. **Crates Externas (`rand`)**
```toml
# Cargo.toml
[dependencies]
rand = "0.8.5"
```

```rust
use rand::Rng;

let secret_number = rand::thread_rng().gen_range(1..=100);
```

### 3. **Loops**
```rust
loop {
    // Código se repete até break
    if guess == secret_number {
        break;
    }
}
```

### 4. **Pattern Matching com `match`**
```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Muito baixo!"),
    Ordering::Greater => println!("Muito alto!"),
    Ordering::Equal => {
        println!("Você acertou!");
        break;
    }
}
```

### 5. **Conversão de Tipos**
```rust
let guess: u32 = guess.trim().parse()
    .expect("Por favor, digite um número!");
```

### 6. **Shadowing**
```rust
let mut guess = String::new();  // String mutável
// ... leitura ...
let guess: u32 = guess.trim().parse()...  // Shadowing com novo tipo
```

---

## 🚀 Como Jogar

```bash
cd 02-guessing-game
cargo run
```

**Saída esperada:**
```
Adivinhe o número!
Digite seu palpite: 50
Muito baixo!
Digite seu palpite: 75
Muito alto!
Digite seu palpite: 63
Você acertou!
```

---

## 💡 Insights Importantes

1. **`use` statements** importam funcionalidades (similar a `import` em Python)
2. **`&` (referências)** emprestam valores sem tomar ownership
3. **`mut` (mutável)** é necessário para modificar variáveis
4. **`.expect()`** lida com possíveis erros (crash controlado com mensagem)
5. **Shadowing** permite reusar nomes de variáveis com tipos diferentes

---

## 🐛 Erros Comuns que Encontrei

1. **Esquecer `mut`** ao declarar variáveis que serão modificadas
2. **Não converter String → número** antes de comparar
3. **Não adicionar `rand` ao `Cargo.toml`** (erro de compilação)
4. **Esquecer `&` ao passar referências** (ownership error)

---

## 🔗 Referências

- [The Book - Cap. 2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
- [Crate `rand`](https://docs.rs/rand/)

---

← [01 - Hello Cargo](../01-hello-cargo/) | **Próximo:** [03 - Variables](../03-variables/) →
