# Curso basico de [Rust-lang](https://www.rust-lang.org)

[![LOGO](rust-logo.jpg)](https://www.rust-lang.org)

###### Fonte: artigo tirado do site [freeCodeCamp](https://www.freecodecamp.org), de [Shaun Hamilton](https://www.freecodecamp.org/news/rust-in-replit/)

## Uma visão geral do Rust

#### Rust, uma linguagem de programação a nivel de sistema

- Lida com detalhes de baixo nivel de gerenciamento de memoria, representacao de dados e simultaneidade.
- A linguagem foi projetada para guiá-lo naturalmente em direção a um código confiável que seja eficiente em termos de velocidade e uso de memória.

#### Principais ferramentas do ecosistema Rust

1. <b>rustc</b> - O copilador do rust que transforma o codigo em binario (Codigo de maquina).
2. <b>rustup</b> - A linha de comando para instalar e atualizar o Rust.
3. <b>cargo</b> - Gerenciador de pacotes do Rust.

### Basico do Rust

#### Variaveis em Rust

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

#### O tipo char no rust

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
calculator 1 + 1
1 + 1 = 2

calculator 138 / 4
138 / 4 = 34.5
```

### Metodologia do Projeto da Calculadora CLI

#### Passo 1 – Criar um novo projeto

- Use Cargo para criar um novo projeto chamado calculadora:

```bash
cargo new calculadora
```

Esse comando cria um novo diretório chamado <b>calculadora</b>, o  <b>Git</b> e um <b>boilerplate</b> útil são adicionado ao seu projeto.

#### O boilerplate inclui

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

#### Passo 3 – Rodando o projeto

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

#### Passo 4 – Argumentos da linha de comando

A biblioteca padrão do <b>Rust</b> vem com um módulo <b>env</b>, que permite acesso aos argumentos de linha de comando passados ​​ao chamar o programa.

As exportações necessárias do módulo env são a função <b>args</b> e a estrutura <b>Args</b>. A função <b>args</b> retorna uma instância da estrutura <b>args</b> e é importada para o escopo do arquivo com:

```rs
use std::env::{args, Args};
```

Para ter uma ideia de como é a estrutura <b>Args</b>, a variável <b>args</b> é impressa no console:

```rs
fn main() {
  let args: Args = args();
  println!("{:?}", args);
}
```

```bash
$ cargo run -- fCC
   Compiling calculator v0.1.0 (/home/runner/Rust-in-Replit/calculator)
    Finished dev [unoptimized + debuginfo] target(s) in 1.71s
     Running `target/debug/calculator`
Args { inner: ["target/debug/toto", "fCC"] }
```

O trecho acima mostra que a estrutura <b>Args</b> contém um campo chamado <b>inner</b> que consiste na localização do binário compilado e os argumentos da linha de comando passados ​​para o programa.

Para acessar os valores dos argumentos, você pode usar o método <b>nth</b> na variável <b>args</b>. O <b>nth</b> método recebe um argumento de <b>index</b>,
e retorna o valor nesse índice envolto em uma opção. Portanto, o valor precisa ser desembrulhado.

```rs
fn main() {
  let mut args: Args = args();

  let first: String = args.nth(1).unwrap();
}
```

A variável <b>args</b> precisa ser declarada como mutável, porque o método <b>nth</b> mutável itera sobre os elementos e remove o elemento acessado.

<b>EX:</b>

```rs
fn main() {
  let mut args: Args = args();

  // O primeiro argumento é a localização do binário compilado, então pule-o
  let first: String = args.nth(1).unwrap();
  // Depois de acessar o segundo argumento, o próximo elemento do iterador se torna o primeiro
  let operator: String = args.nth(0).unwrap();
  let second: String = args.nth(0).unwrap();

  println!("{} {} {}", first, operator, second);
}
```

```bash
$ cargo run -- 1 + 1
   Compiling calculator v0.1.0 (/home/runner/Rust-in-Replit/calculator)
    Finished dev [unoptimized + debuginfo] target(s) in 1.71s
     Running `target/debug/calculator 1 + 1` 
```

#### Passo 5 – transformar Strings em Números

A primeira e a segunda variáveis ​​são <b>strings</b> e você precisa transforma-las em números. A estrutura <b>strings</b> implementa o método <b>parse</b>, que recebe uma anotação de tipo e retorna um <b>Result</b> contendo o valor analisado.

```rs
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first: String = args.nth(1).unwrap();
  let operator: String = args.nth(0).unwrap();
  let second: String = args.nth(0).unwrap();

  let first_number = first.parse::<f32>().unwrap();
  let second_number = second.parse::<f32>().unwrap();

  println!("{} {} {}", first_number, operator, second_number);
}
```

O método <b>parse</b> acima usa a sintaxe do <b>turbofish</b> para especificar o tipo para tentar transformar a <b>string</b>.

---

#### Passo 6 – Executando operações aritméticas básicas

<b>Rust</b> usa os operadores padrão para  realizar adições, subtrações, multiplicações e divisões.

Para lidar com as operações, você define uma função chamada <b>operate</b> que recebe três argumentos:
O operador como um <b>char</b>, e dois números como <b>f32</b>s. A função <b>operate</b> retorna um <b>f32</b> como resultado.

<b>EX:</b>

```rs
fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}
```

A expressão <b>match</b> funciona de maneira semelhante a uma instrução <b>switch</b> em outra linguagem. A expressão <b>match</b> recebe um valor e uma lista de <b>casos</b>. Cada <b>casos</b> é um padrão de valor e uma bloco. O padrão é um valor para corresponder e o bloco é o código a ser executado se o padrão corresponder. O padrão <b>_</b> é um curinga, agindo como uma cláusula <b>else</b>.
O <b>caso</b> de multiplicação inclui a comparação <b>OR</b> para permitir que <b>casos</b> para X e x sejam tratados.

Agora, para chamar operar com o operador, você precisa primeiro convertê-lo em um char. Você faz isso com o método chars na estrutura String que retorna um iterador sobre os caracteres na string. Em seguida, o primeiro caractere é desempacotado:

<b>EX:</b>

```rs
fn main() {
  let mut args: Args = args();

  let first: String = args.nth(1).unwrap();
  let operator: char = args.nth(0).unwrap().chars().next().unwrap();
  let second: String = args.nth(0).unwrap();

  let first_number = first.parse::<f32>().unwrap();
  let second_number = second.parse::<f32>().unwrap();
  let result = operate(operator, first_number, second_number);

  println!("{} {} {}", first_number, operator, second_number);
}
```

O retorno da função <b>operate</b> sera alocada na variavel <b>result</b>.

---

#### Passo sete – Formate a saída

Para obter a saida desejada, as variaveis <b>first_number</b>, <b>second_number</b> e <b>result</b> precisam ser formatadas. Você pode usar o método macro <b>format!</b> para criar uma <b>String</b> a partir de uma <b>string</b> e uma lista de argumentos.

```rs
fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}
```

#### Passo 8 – Junte tudo

```rs
use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first: String = args.nth(1).unwrap();
  let operator: char = args.nth(0).unwrap().chars().next().unwrap();
  let second: String = args.nth(0).unwrap();

  let first_number = first.parse::<f32>().unwrap();
  let second_number = second.parse::<f32>().unwrap();
  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}
```

Para criar o executável, você precisa rodar o seguinte comando:

```bash
$ cargo build --release
   Compiling calculadora v0.1.0 (/home/junior/Code Projects/Rust-Lang/Curso_basico_de_RustLang/calculadora)
    Finished release [optimized] target(s) in 0.29s
```

A flag <b>--release</b> diz a o <b>Cargo</b> para compilar o binario para o modo <b>release</b>. Isso reduzirá o tamanho do binário e também removerá qualquer informação de depuração.

O binario sera criado no diretorio <b>target/release</b>. Para rodar o binario, você precisa executar o seguinte comando:

```bash
$ target/release/calculadora 1 + 1
1 + 1 = 2
```

---

# Projeto #2 – Construir um Combinador de Imagens em Rust

#### Resultado do projeto

- No final deste projeto, você poderá combinar duas imagens usando a linha de comando.

Aqui está um exemplo de uma entrada esperada:

```bash
combiner ./image1.png ./image2.png ./output.png
```

Para um exemplo de saida, use a imagem desse artigo: ☝️

### Metodologia desse projeto

#### Passo 1 - Criar um novo projeto

Use o <b>Cargo</b> para criar um novo projeto chamado <b>combiner</b>:

```bash
cargo new combiner
```

#### Passo 2 - Adicionar um modulo novo chamado Args

Para evitar que o arquivo <b>main.rs</b> fique muito pessado, crie um novo modulo chamado <b>Args.rs</b> no diretorio <b>src/</b>.
Dentro de <b>Args.rs</b> crie uma função chamada <b>get_nth_arg</b> que recebe um <b>usize</b>, <b>n</b>, e retorna uma <b>String</b>. Então, apartir do modulo <b>std::env</b>, chame a função <b>args</b> e encadeie o <b>nth</b> método para obter o <b>nth</b> argumento, desempacotando o valor:

<b>EX:</b>

```rs
fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}
```

Defina uma estrutura pública chamada <b>Args</b> que consiste em três campos públicos de tipo <b>String</b>:<b>image_1</b>, <b>image_2</b> e <b>output</b>:

<b>EX:</b>

```rs
pub struct Args {
  pub image_1: String,
  pub image_2: String,
  pub output: String,
}
```

Declare a estrutura e seus campos como públicos com a palavra-chave <b>pub</b> para que você possa acessá-los de fora do arquivo <b>args.rs.</b>

Por fim, você pode usar a função <b>get_nth_arg</b> para criar uma nova estrutura <b>Args</b> em uma nova função:

<b>EX:</b>

```rs
impl Args {
  pub fn new() -> Self {
    Args {
      image_1: get_nth_arg(1),
      image_2: get_nth_arg(2),
      output: get_nth_arg(3),
    }
  }
}
```

O arquivo <b>args.rs</b> deve ficar  como o seguinte:

```rs
pub struct Args {
  pub image_1: String,
  pub image_2: String,
  pub output: String,
}

impl Args {
  pub fn new() -> Self {
    Args {
      image_1: get_nth_arg(1),
      image_2: get_nth_arg(2),
      output: get_nth_arg(3),
    }
  }
}

fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}
```

---

#### Passo 3 – Importar e usar o módulo args

Dentro de <b>main.rs</b>, você precisa declarar o arquivo <b>args.rs</b> como um módulo. Então, para usar a estrutura <b>Args</b>, você precisa importá-la:

<b>EX:</b>

```rs
mod args;
use args::Args;

fn main() {
  let args = Args::new();
  println!("{:?}", args);
}
```

Mas testar o código revela um erro:

<b>EX:</b>

```bash
$ cargo run -- arg1 arg2 arg3
   Compiling combiner v0.1.0 (/home/runner/Rust-in-Replit/combiner)
error[E0277]: `args::Args` doesn't implement `Debug`
  --> src/main.rs:12:20
   |
12 |   println!("{:?}", args);
   |                    ^^^^ `args::Args` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `args::Args`
   = note: add `#[derive(Debug)]` or manually implement `Debug`
   = note: required by `std::fmt::Debug::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `combiner`

To learn more, run the command again with --verbose.
```

Da mesma forma que as funções são implementadas para <b>structs</b>, as <b>traits</b> podem ser implementadas para <b>structs</b>. No entanto, o <b>traits</b> <b>Debug</b> é especial, pois pode ser implementado usando atributos:

<b>EX:</b>

```rs
#[derive(Debug)]
pub struct Args {
  pub image_1: String,
  pub image_2: String,
  pub output: String,
}
```

A característica <b>Debug</b> foi derivada para a estrutura <b>Args</b>. Isso significa que o <b>traits</b> <b>Debug</b> é implementado automaticamente para a estrutura, sem que você precise implementá-lo manualmente 🚀.

Agora, executando o código funciona:

```bash
$ cargo run -- arg1 arg2 arg3
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/combiner arg1 arg2 arg3`
Args { image_1: "arg1", image_2: "arg2", output: "arg3" }
```

#### Passo 4 – Adicione uma Crate

Da mesma forma que outras linguagens têm bibliotecas ou pacotes, <b>Rust</b> tem <b>Crates</b>. Para codificar e decodificar imagens, você pode usar a <b>crate</b> <b>image</b>.

Adicione a <b>crate</b> de imagem com a versão <b>0.23.14</b> ao arquivo <b>Cargo.toml</b>:

<b>EX:</b>

```toml
[package]
name = "combiner"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
image = "0.23.14"
```

Agora, quando o <b>cargo</b> for chamada, Cargo irá buscar e instalar a <b>crate</b> <b>image</b>.

---

#### Passo 5 – Ler um arquivo de imagem

A <b>crate</b> de <b>image</b> vem com um módulo <b>io</b> incluindo uma estrutura <b>Reader</b>. Essa estrutura implementa uma função <b>open</b> que leva um caminho para um arquivo de imagem e retorna um <b>Result</b> contendo um leitor. Você pode formatar e decodificar este leitor para produzir o formato da imagem <b>(por exemplo, PNG, JGP e assim por diante)</b> e os dados da imagem.
Crie uma função chamada <b>find_image_from_path</b> para abrir o arquivo de imagem a partir de um argumento de <b>path</b>:

<b>EX:</b>

```rs
fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
  let image_format: ImageFormat = image_reader.format().unwrap();
  let image: DynamicImage = image_reader.decode().unwrap();
  (image, image_format)
}
```

As variaveis <b>image</b> e <b>image_format</b> iram retorna uma <b>tupla</b>.

Inclua os <b>imports</b> necessários:

```rs
use image::{ io::Reader, DynamicImage, ImageFormat };

fn main() {
  // ...
  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);
}
```

Dentro de <b>main</b>, a tupla retornada pode ser desestruturada em duas novas variáveis ​​para cada caminho de imagem.

---

#### Passo 6 – Lidar com Erros com Result

É importante ser capaz de lidar com os erros que surgem. Por exemplo, você pode ter um caso em que duas imagens de formatos diferentes são fornecidas como argumentos para combinar.

Uma maneira semântica de lidar com esse erro é retornar um <b>Result</b> que pode consistir em um <b>Ok</b> ou um <b>Err</b>.

<b>EX:</b>

```rs
fn main() -> Result<(), ImageDataErrors> {
  let args = Args::new();
  println!("{:?}", args);

  let (image_1, image_1_format) = find_image_from_path(args.image_1);
  let (image_2, image_2_format) = find_image_from_path(args.image_2);

  if image_1_format != image_2_format {
    return Err(ImageDataErrors::DifferentImageFormats);
  }
  Ok(())
}
```

A função <b>main</b> retorna um <b>Err</b> contendo uma enum com a variante de unidade <b>DifferentImageFormats</b> se os dois formatos de imagem não forem iguais. Caso contrário, retorna um <b>Ok</b> com uma <b>tupla</b> vazia.

O enum é definido como:

<b>EX:</b>

```rs
enum ImageDataErrors {
  DifferentImageFormats,
}
```

---

#### Passo 7 – Redimensione as imagens para combinar

Para facilitar a combinação das imagens, você redimensiona a imagem maior para corresponder à imagem menor.

Primeiro, você pode encontrar a menor imagem usando o método de <b>dimensions</b> que retorna a largura e a altura da imagem como uma tupla. Essas tuplas podem ser comparadas e a menor retornada:

<b>EX:</b>

```rs
fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}
```

Os valores da tupla são acessados ​​usando notação de ponto da indexação baseada em zero.

Se a <b>imagem_2</b> for a menor imagem, a <b>imagem_1</b> precisará ser redimensionada para corresponder às menores dimensões. Caso contrário, <b>image_2</b> precisa ser redimensionado.

<b>EX:</b>

```rs
fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);

  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
    (image_1, image_2.resize_exact(width, height, Triangle))
  }
}
```

O método <b>resize_exact</b> implementado na estrutura <b>DynamicImage</b> empresta a imagem de forma mutável e, usando os argumentos <b>width</b>, <b>height</b> e <b>FilterType</b>, redimensiona a imagem.

Usando o retorno da função <b>standardise_size</b>, você pode redeclarar as variáveis <b>​​image_1</b> e <b>​​image_2</b>:

```rs
use image::{ io::Reader, DynamicImage, ImageFormat, imageops::FilterType::Triangle };

fn main() -> Result<(), ImageDataErrors> {
  // ...
  let (image_1, image_2) = standardise_size(image_1, image_2);
  Ok(())
}
```

---

#### Passo 8 – Crie uma imagem Floating

Para manipular a saída, crie uma estrutura temporária para conter os metadados da imagem de saída.

Defina um struct chamado <b>FloatingImage</b> para conter a <b>width</b>, <b>height</b> e a  data da imagem, bem como o <b>name</b> do arquivo de saída:

```rs
struct FloatingImage {
  width: u32,
  height: u32,
  data: Vec<u8>,
  name: String,
}
```

Em seguida, implemente uma nova função para <b>FloatingImage</b> que recebe valores para <b>width</b>, <b>height</b> e <b>name</b> da imagem de saída:

```rs
impl FloatingImage {
  fn new(width: u32, height: u32, name: String) -> Self {
    let buffer_capacity = 3_655_744;
    let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
    FloatingImage {
      width,
      height,
      data: buffer,
      name,
    }
  }
}
```

Como você ainda não criou os dados para a imagem, crie um buffer na forma de um <b>Vec</b> de <b>u8s</b> com capacidade de <b>3.655.744 (956 x 956 x 4)</b>. A sintaxe ```<number>_<number>``` é a numeração de fácil leitura do <b>Rust</b> que separa o número em grupos ou três dígitos.
Use os valores de <b>width</b> e <b>height</b> da variável <b>image_1</b> para criar uma instância da <b>FloatingImage</b> e use o terceiro argumento armazenado em <b>args</b> para definir o nome da <b>FloatingImage</b>:

<b>EX:</b>

```rs
fn main() -> Result<(), ImageDataErrors> {
  // ...
  let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);
  Ok(())
}
```

Desclare as variaveis de saida como mutáveis para que você possa manipular os campos de dados posteriormente.

---
