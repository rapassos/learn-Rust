# 🔢 Capítulo 3 — Variáveis e Tipos de Dados

> Exploração profunda do sistema de tipos do Rust, mutabilidade, shadowing e conceitos fundamentais de gerenciamento de memória.

## 🎯 Conceitos do Capítulo

### 1. **Variáveis e Mutabilidade**

Por padrão, variáveis em Rust são **imutáveis**:

```rust
let x = 5;
x = 6;  // ❌ ERRO! x é imutável
```

Para permitir mudanças, use `mut`:

```rust
let mut x = 5;
x = 6;  // ✅ OK!
```

**Por que imutabilidade por padrão?**  
Previne bugs relacionados a mudanças inesperadas de estado — segurança em tempo de compilação.

---

### 2. **Constantes vs. Variáveis**

```rust
const MAX_POINTS: u32 = 100_000;  // Tipo DEVE ser anotado
```

**Diferenças:**
- Constantes **nunca** podem ser mutáveis
- Constantes podem ser declaradas em **qualquer escopo** (incluindo global)
- Constantes só aceitam **expressões constantes** (avaliadas em compile time)

---

### 3. **Shadowing**

Você pode redeclarar uma variável com o mesmo nome:

```rust
let x = 5;
let x = x + 1;  // Shadowing — novo binding
let x = x * 2;  // Outro shadowing

println!("{x}");  // 12
```

**Shadowing vs. `mut`:**
```rust
// Com shadowing, pode mudar o TIPO
let spaces = "   ";
let spaces = spaces.len();  // ✅ OK — agora é número

// Com mut, o TIPO é fixo
let mut spaces = "   ";
spaces = spaces.len();  // ❌ ERRO — tipo incompatível
```

---

### 4. **Tipos de Dados**

#### **Escalares** (valores únicos)

##### Inteiros
```rust
let a: i8 = -127;      // 8 bits com sinal
let b: u32 = 100_000;  // 32 bits sem sinal
let c = 98_222;        // inferido como i32 (padrão)
```

| Tamanho | Com sinal | Sem sinal |
|---------|-----------|-----------|
| 8 bits  | `i8`      | `u8`      |
| 16 bits | `i16`     | `u16`     |
| 32 bits | `i32`     | `u32`     |
| 64 bits | `i64`     | `u64`     |
| 128 bits| `i128`    | `u128`    |
| arch    | `isize`   | `usize`   |

##### Ponto Flutuante
```rust
let x = 2.0;      // f64 (padrão)
let y: f32 = 3.0; // f32
```

##### Booleanos
```rust
let t = true;
let f: bool = false;
```

##### Caracteres
```rust
let c = 'z';
let emoji = '😻';  // Unicode! (4 bytes)
```

---

#### **Compostos** (múltiplos valores)

##### Tuplas
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// Desestruturação
let (x, y, z) = tup;

// Acesso por índice
let five_hundred = tup.0;
```

##### Arrays
```rust
let a = [1, 2, 3, 4, 5];
let months = ["Janeiro", "Fevereiro", /* ... */];

// Array com tipo e tamanho
let a: [i32; 5] = [1, 2, 3, 4, 5];

// Array com valores repetidos
let a = [3; 5];  // [3, 3, 3, 3, 3]

// Acesso
let first = a[0];
```

**⚠️ Arrays têm tamanho fixo** (diferente de vetores `Vec`)

---

## 💡 Aprendizados Principais

1. **Imutabilidade padrão** reduz bugs — mude apenas quando necessário
2. **Shadowing** é útil para transformações de valores mantendo nome semântico
3. **Tipos anotados** são necessários quando compilador não consegue inferir
4. **Integer overflow** em debug mode causa **panic**, em release mode faz **wrap**
5. **Caracteres são Unicode** (4 bytes cada), não ASCII (1 byte)

---

## 🐛 Erros Comuns

```rust
// ❌ Tentar mutar sem `mut`
let x = 5;
x = 6;  // erro: cannot assign twice to immutable variable

// ❌ Acessar índice inválido de array
let a = [1, 2, 3];
let element = a[10];  // panic em runtime!

// ❌ Confundir tupla com array
let tup = (1, 2, 3);
let x = tup[0];  // erro: não pode indexar tupla assim
```

---

## 🔬 Experimentos Feitos

- Testar overflow de inteiros (wrap vs panic)
- Shadowing com mudança de tipo
- Diferença de performance entre `i32` e `i64`
- Unicode em caracteres (`char`)

---

## 🔗 Referências

- [The Book - Cap. 3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [The Book - Cap. 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html)

---

← [02 - Guessing Game](../02-guessing-game/) | **Próximo:** 04 - Ownership (em breve) →
