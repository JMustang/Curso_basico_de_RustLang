###### Fonte: artigo tirado do site [freeCodeCamp](https://www.freecodecamp.org), de [Shaun Hamilton](https://www.freecodecamp.org/news/rust-in-replit/).
# Curso basico de [Rust-lang](https://www.rust-lang.org)
## Uma visão geral do Rust

#### Rust, uma linguagem de programação a nivel de sistema.

- Lida com detalhes de baixo nivel de gerenciamento de memoria, representacao de dados e simultaneidade.
- A linguagem foi projetada para guiá-lo naturalmente em direção a um código confiável que seja eficiente em termos de velocidade e uso de memória.

#### Principais ferramentas do ecosistema Rust.

1. <b>rustc</b> - O copilador do rust que transforma o codigo em binario (Codigo de maquina).
2. <b>rustup</b> - A linha de comando para instalar e atualizar o Rust.
3. <b>cargo</b> - Gerenciador de pacotes do Rust.


### Basico do Rust
#### Variaveis em Rust.

Você pode declarar uma variaveu usando, <b>let</b>, <b>const</b> ou <b>static</b>

EX:
```rs
let minha_variavel = 0;
const minha_constante: u8 = 0;
static meu_static: u8 = 0;
```

Por padrão, todas as variáveis sao imutáveis, mas Você pode transforma em mutável usando a palavra-chave <b>mut</b>.

EX:
```rust
let mut minha_variavel_mutavel = 0;
```

Convenções rust.

```sql
OBJECT     | CASING
---------- | ----------
Variables  | snake_case
Functions  | snake_case
Files      | snake_case
Constants  | SCREAMING_SNAKE_CASE
Statics    | SCREAMING_SNAKE_CASE
Types      | PascalCase
Traits     | PascalCase
Enums      | PascalCase
```


Como rust é tipado estaticamente, você precisará digitar variáveis ​​explicitamente - a menos que a variável seja declarada com <b>let</b> e o tipo possa ser inferido.
