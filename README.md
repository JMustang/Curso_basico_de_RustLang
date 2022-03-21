# Curso basico de [Rust-lang](https://www.rust-lang.org)


[![LOGO](rust-logo.jpg)](https://www.rust-lang.org)

###### Fonte: artigo tirado do site [freeCodeCamp](https://www.freecodecamp.org), de [Shaun Hamilton](https://www.freecodecamp.org/news/rust-in-replit/).



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

---

Você pode declarar uma variaveu usando, <b>let</b>, <b>const</b> ou <b>static</b>

<b>EX:</b>

```rs
let minha_variavel = 0;
const minha_constante: u8 = 0;
static meu_static: u8 = 0;
```

Por padrão, todas as variáveis sao imutáveis, mas Você pode transforma em mutável usando a palavra-chave <b>mut</b>.

<b>EX:</b>

```rust
let mut minha_variavel_mutavel = 0;
```

Convenções rust.

<b>EX:</b>

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

---

#### função em Rust

- Função em rust pode ser declaradas usando a palavra-chave <b>fn</b>.

<b>EX:</b>

```rs
fn main() {
  // comentario de codigo
}
```

- As funções retornam usando a palavra-chave return e você precisa especificar explicitamente o tipo de retorno de uma função, a menos que o tipo de retorno seja uma tupla vazia ():

<b>EX:</b>

```rs
fn main() ->{}{ // Tipo de retorno desnecessário
  minha_func();
}

fn minha_func()-> u8 {
  return 0;
}
```

- As funções também retornam uma expressão sem o ponto e vírgula:

<b>EX:</b>

```rs
fn minha_func() -> u8 {
  0
}
```

- Os parâmetros das funções são passados usando a sintaxe <b>":"</b>.

<b>EX:</b>

```rs
fn main() {
  let _variavel_nao_utilizada = minha_func(10);
}

fn minha_func(x: u8) -> i32 {
  x as i32
}
```

- O sublinhado antes de um nome de variável é uma convenção para indicar que a variável não é usada. A palavra-chave <b>"as"</b> afirma o tipo da expressão, desde que a conversão de tipo seja válida.

---

#### Strings e Slices em Rust

- Um ponto em comum que confunde os novatos <b>Rustacians</b>, é a diferenca entre <b>Strings</b> e o tipo <b>str</b>.

<b>EX:</b>

```rs
let meu_str: &str = "Ola mundo!";

let minha_string: String = String::from("Ola Mundo!");
```
- No exemplo acima, <b>meu_str</b> é uma referencia para uma <b>String</b> literal, e <b>minha_string</b> é uma instacia da <b>String</b>.
- Uma distinção importante entre elas é que <b>meu_str</b> é armazenada em pilha e <b>minha_string</b> é alocada em pilha. Isso significa que o valor de <b>meu_str</b> não pode mudar e seu tamanho e fixo, enquanto <b>minha_string</b> pode ter um tamanho desconhecido em tempo de compilação.
- A <b>String</b> literal tambem e conhecida como fatia de string. Isso ocorre porque um <b>&str</b> se refere a parte de uma string. Geralmente, é assim que <b>arrays</b> e <b>Strrings</b> assemelham-se.

<b>EX:</b>

```rs
let minha_string = String::from("The quick brown fox");
let meu_str: &str = &minha_string[4..9]; //"Rapido"

let meu_arr: [usize; 5] = [1,2,3,4,5];
let meu_arr_fatiado: &[usize] = &meu_arr[0..3]; // [1,2,3]
```

- O <b>[T; n]</b> é usado para criar um array de <b>n</b> elementos de tipo <b>T</b>.

---

#### O tipo char no rust.
- Um <b>char</b> é um USV (Unicode Scalar Value), que é representado por um valor unicode como <b>'∞'</b>. Você deve pensar em uma coleção ou array de caracteres como uma <b>string</b>.

EX:
```rs
let meu_str: &str = "Ola, Mundo!"

let colecao_de_chars: &str = meu_str.chars().as_str();
```
---

#### Tipo Number em rust

- Existem varios tipos de <b>Numbers</b> em <b>rust</b>:

1.Inteiros não declarado:<b>u8, u16, u32, u64, u128</b> 
2.Inteiros declarado:<b>i8, i16, i32, i64, i128</b>
3.Números Float:<b>f32, f64</b>

- Inteiros não declarado, representam apenas números inteiros positivos.
- Inteiros declarado, representam números inteiros positivos e negativos.
- Float representam apenas frações positivas e negativas.
---

#### Structs no Rust

- Um <b>Struct </b> é um tipo de dados personalizado usado para agrupar dados  relacionados. Você já encontrou um struct na seção <b>Strings e Slices</b>:

<b>EX:</b>

```rs
struct String {
  vec: Vec<u8>,
}
```

- A <b>String struct</b> consiste em um campo <b>vec</b>, que é um <b>Vec</b> de u8s. O <b>Vec</b> é um array de tamanho dinâmico.

- Uma instância de um struct é então declarada dando valores aos campos:
<b>EX:</b>

```rs
struct MeuStruct {
  field_1: u8,
}

let meu_struct = MeuStruct { field_1: 0, };
```
- Anteriormente, a <b>struct String</b> era usado com sua função <b>from</b> para criar uma <b>String</b> a partir de um &str. Isso é possivel porque a função <b>from</b> é implementada para a <b>String</b>:

<b>EX:</b>

```rs
iml String {
  fn from(s: &str) -> self {
    String {
      vec: Vec::from(s.as_bytes()),
    }
  }
}
```

- Você usa a palavra-chave <b>Self</b> no lugar do tipo da <b>struct</b>.

Os <b>structs</b> também podem ter outras variantes:

<b>EX:</b>

```rs
struct MinhaUnidadeStruct;
struct MinhaTuplesStruct(u8, u8);
```

---
#### Enums em Rust
- Semelhante a outras linguagens, enums são úteis para atuar como tipos e valores.

<b>EX:</b>

```rs
enum meusErros {
  CabecaCansada,
  TimeOfDay(String)
  SemCafe,
}

fn work() -> Result<(), meusErros> { // Result também é um enum
  if state == "Faltando ponto-e-vírgula" {
    Err(meusErros::CabecaCansada)
  } else if state == "06:00" {
    Err(meusErros::TimeOfDay("é muito cedo para trabalhar".to_string()))
  } else if state == "22:00" {
    Err(meusErros::TimeOfDay("é muito tarde para trabalhar".to_string()))
  } else if state == "vaziu" {
    Err(meusErros::SemCafe)
  } else {
    ok(())
  }
}
```

---

#### Macros em Rust
- Um macro é semelhante a uma função, mas você pode pensar nela como um pedaço de código que escreve outro código. Por enquanto, as principais diferenças entre uma função e uma macro a serem lembradas são:

1. As macros são chamadas usando um bang (!).
2. Macros podem receber um número variável de argumentos, enquanto funções em Rust não podem.

- Uma das macros mais comuns é a println! macro, que imprime no console:

<b>EX:</b>

```rs
let meu_str = "Ola, Mundo!";
println!("{}", meu_str);
```
- Você usa a sintaxe <b>{}</b> para inserir uma variável em uma string.

- Outra macro comum é o <b>panic!</b>. Entrar em pânico é a maneira de Rust 'errar'. É sábio pensar em um <b>panic!</b> em <b>Rust</b> como um erro mal tratado. A macro aceita um literal de string e entra em pânico com essa mensagem.

<b>EX:</b>

```rs
let eu_sou_um_erro = true;

if (eu_sou_um_erro) {
  panic!("Existe um erro");
}
```

```zsh 
# cargo é o NPM (package manage) do Rust
$ cargo run
   Compiling fcc-rust-in-replit v0.1.0 (/home/runner/Rust-in-Replit)
    Finished dev [unoptimized + debuginfo] target(s) in 1.66s
     Running `target/debug/calculator`
thread 'main' panicked at 'There was an error', src/main.rs
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
---

#### Ownership em Rust
- Um conceito importante em <b>Rust</b> é a <b>Ownership</b>. Existem três regras principais de <b>Ownership</b>:

1. Cada valor em <b>Rust</b> tem uma variável que é chamada de <b>owner</b>.
2. Só pode haver um <b>owner</b> de cada vez.
 3. Quando o <b>owner</b> sair do escopo, o valor será descartado.

- É assim que o Rust se safa de não ter um <b>garbage collector</b>, ao mesmo tempo em que não exige que o programador gerencie explicitamente a memória. Aqui está um exemplo de <b>Ownership</b>:

<b>EX:</b>

```rs
fn main() {// first_string ainda não foi declarado -> não tem valor
  let first_string = String::from("freeCodeCamp"); // first_string agora e Ownership de um valor "freeCodeCamp"
  let second_string = first_string; // second_string assume a Ownership do valor "freeCodeCamp"

  println!("Hello, {}", first_string) // first_string NÃO é válido, porque o valor foi movido para second_string
}
```

- Como o macro <b>println!</b> tenta se referir a uma variável inválida, esse código não compila. Para corrigir isso, em vez de mover o valor de <b>first_string</b> para <b>second_string</b>, <b>second_string</b> pode receber uma referência para <b>first_string</b>:

<b>EX:</b>

```rs
fn main() {
  let first_string: String = String::from("freeCodeCamp");
  let second_string: &String = &first_string; // first_string ainda é o owner do valor "freeCodeCamp"
  println!("Hello, {}", first_string)
}
```

- O <b>E</b> comercial <b>(&)</b> indica que o valor é uma referência. Ou seja, <b>second_string</b> não se apropria mais de <b>"freeCodeCamp"</b>, mas, em vez disso, aponta para o mesmo ponto na memória que <b>first_string</b>.

---

# Projeto #1 – Construir uma calculadora CLI em Rust
#### Resultado do projeto
- No final deste projeto, você será capaz de realizar operações aritméticas básicas em números usando a linha de comando.

Exemplos de entrada e saída esperadas são assim:

```bash
$ calculator 1 + 1
$ 1 + 1 = 2

$ calculator 138 / 4
$ 138 / 4 = 34.5
```

### Metodologia do Projeto da Calculadora CLI
#### Passo 1 – Criar um novo projeto
- Use Cargo para criar um novo projeto chamado calculadora:

```bash
$ cargo new calculadora
```
Esse comando cria um novo diretório chamado <b>calculadora</b>, o  <b>Git</b> e um <b>boilerplate</b> útil são adicionado ao seu projeto.

#### O boilerplate inclui:
1. <b>Cargo.toml</b> - O arquivo de manifesto usado pelo Cargo para gerenciar os metadados do seu projeto
2. <b>src</b> - O diretorio onde vai ficar o seu codigo.
3. <b>src/main.rs</b> - O arquivo padrão que o Cargo usa como ponto de entrada do aplicativo

---

#### Passo 2 – Entendendo a sintax
O arquivo <b>Cargo.toml</b> contém o seguinte:

```toml
[package]
name = "calculator"
version = "0.1.0"
edition = "2018"

[dependencies]
```
O [package] indica os metadados do seu projeto.

O cabeçalho [dependencies] indica as <b>crates</b> das quais seu projeto depende. As <b>crates</b> são como bibliotecas externas.

O arquivo <b>main.rs</b> contém o seguinte:

```rs
fn main() {
  println!("Hello, world!");
}
```

Este arquivo contém uma declaração de função com o handle <b>main</b>. Por padrão, <b>rustc</b> chama a função <b>main</b> primeiro sempre que o executável for chamado.

println! é uma macro interna que "printa" algo no console.

---

#### Passo 3 – Rodando o projeto.
Você pode usar o <b>Cargo</b> para executar o código do seu projeto:

```bash
# Dentro do diretorio /calculadora
$ cargo run
   Compiling fcc-rust-in-replit v0.1.0 (/home/runner/Rust-in-Replit-1)
    Finished dev [unoptimized + debuginfo] target(s) in 0.80s
     Running `target/debug/calculator`
Hello, world!
```

Ou Você pode usa <b>rustc</b> para copilar o projeto,e roda o binario:

```bash
# Dentro do diretorio /calculadora
$ rustc src/main.rs
$ ./main
Hello, world!
```
---