# Compilador de expressões matemáticas em Rust

## Informações do autor
- **Nome:** Gabriel Hagui dos Santos
- **Matrícula:** 11202020238
- **Curso:** Ciência da Computação
- **Instituição:** Universidade Federal do ABC
- **Disciplina:** Compiladores
- **Linguagem de implementação:** Rust

## Descrição

Este projeto consiste na implementação de um compilador simples para uma linguagem de matemática básica. O compilador é capaz de realizar operações aritméticas básicas, como:

- Soma
- Subtração
- Multiplicação
- Divisão

Além disso, suporta o uso de parênteses para definir a precedência das operações e operadores de comparação, incluindo:

- Maior que
- Menor que
- Maior ou igual a
- Menor ou igual a
- Igual a
- Diferente de

## Por que Rust?
A linguagem Rust foi escolhida por ser moderna, segura e eficiente. Ao contrário de Java, Rust não utiliza um garbage collector, mas assegura a segurança de memória através do conceito de ownership. Isso ajuda a evitar problemas comuns em C e C++, oferecendo desempenho semelhante a essas linguagens.

Além disso, a Fundação Linux, a Microsoft e a Google estão investindo em Rust para o desenvolvimento de drivers, sistemas internos e até mesmo no Android, o que evidencia ainda mais o potencial da linguagem. É a oportunidade perfeita para aprender Rust e explorar suas capacidades.

## Etapas do compilador
- Entrada de usuário (expressão matemática)
- Análise léxica
- Análise sintática
- Análise semântica
- Saída do resultado

## Explicação do código
O código foi dividido em módulos para facilitar a organização e manutenção. O módulo `lexer` é responsável por realizar a análise léxica, enquanto o módulo `parser` é responsável pela análise sintática e semântica.

No código principal, a função `main` é responsável por receber a entrada do usuário, chamar a função `lexer::lex` para realizar a análise léxica e, em seguida, chamar a função `parser::parse` para realizar a análise sintática e semântica. Para começar, o código principal exibe uma mensagem ao usuário e entra em um loop para que o usuário possa digitar várias expressões sem precisar reiniciar o programa.
No código `main.rs`:
```rust
fn main() {
    loop {
        let mut input = String::new();

        println!("Digite uma expressão aritmética ou 'exit' para sair ou 'help' para ver as expressões suportadas:");
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        // ( ... )
        }
    }
}
```
Essa parte está em loop para que o usuário possa digitar várias expressões sem precisar reiniciar o programa. Em seguida, passa por condições para verificar se o usuário deseja sair do programa ou se deseja ver as expressões suportadas. Segue abaixo o código que verifica se o usuário deseja sair ou ver as expressões suportadas:
```rust
// ( ... )
if input == "exit" {
            break;
        }
else if input == "help" {
    println!("  Expressões aritméticas suportadas: +, -, *, /, (, ), ==, !=, >, <, >=, <=, números inteiros e decimais.");
    println!("  Caso compare dois números, o resultado será 1.0 se a comparação for verdadeira e 0.0 se for falsa.\n");
        }
else {
    let tokens = lexer::lex(input);
    let expr = parser::parse(&tokens);
    match expr {
        Ok(expr) => {
            println!("  Tokens gerados: {:?}\n", tokens);
            println!("  Árvore sintática: {:?}\n", expr);
            let result = parser::avaliar(&expr);
            println!("  Resultado: {:?}\n", result);
        }
        Err(e) => {
            println!("{}\n", e);
        }
    }
}
```
O caso `help` exibe as expressões suportadas pelo compilador. O 'exit' encerra o programa. Se o usuário digite a expressão, o programa chama a função `lexer::lex(input)` para realizar a análise léxica e a função `parser::parse(tokens)` para realizar a análise sintática e semântica. Em seguida, exibe os tokens gerados, a árvore sintática e o resultado da expressão.

Ao chamar a função `lexer::lex(input)`, no código `lexer.rs`, o código passa a expressão do usuário em formato de string e cria um vetor de tokens. Segue o código abaixo:
```rust
pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut caracteres = input.chars().peekable();
    tokens.push(Token::INICIO);
    
    // ( ... )
}
```
Em seguida, faz um push de INICIO no vetor de tokens (`tokens.push(Token::INICIO);`). Token é um enumeration que representa os tipos de tokens suportados pelo compilador. Segue o código do enumeration:
```rust
pub enum Token {
    INICIO,
    Numero(f64),
    Operador(char),
    Comparador(String),
    Parentese(char),
    FIM,
}
```
O enumeration Token possui os seguintes tipos de tokens:
- `INICIO`: Representa o início da expressão
- `Numero(f64)`: Representa um número inteiro ou decimal em formato de ponto flutuante
- `Operador(char)`: Representa um operador aritmético (+, -, *, /) em formato de caractere
- `Comparador(String)`: Representa um operador de comparação (==, !=, >, <, >=, <=) em formato de string
- `Parentese(char)`: Representa um parêntese de abertura ou fechamento (, ) em formato de caractere
- `FIM`: Representa o fim da expressão

Em seguida o código faz um loop para percorrer os caracteres da expressão do usuário e identificar os tokens correspondentes.
```rust
while let Some(&caractere) = caracteres.peek() {
        match caractere {( ... ) }
        }
```
Essa parte do código cima é parecido com um switch case, onde ele verifica o caractere atual e faz a ação correspondente. Segue o código que verifica o caractere atual:
```rust
'=' => {
    caracteres.next();
    if let Some(&next_char) = caracteres.peek() {
        if next_char == '=' {
            tokens.push(Token::Comparador("==".to_string()));
            caracteres.next();
        } else {
            tokens.push(Token::Operador(caractere));
        }
    } else {
        tokens.push(Token::Operador(caractere));
    }
}
```
No código acima, se o caractere atual for '=', o código verifica o próximo caractere. Se o próximo caractere for '=', ele adiciona o token Comparador("==") ao vetor de tokens. Caso contrário, ele adiciona o token Operador('=') ao vetor de tokens.

```rust
'>' | '<' | '!' => {
    caracteres.next();
    let mut comparador = caractere.to_string();
    if let Some(&next_char) = caracteres.peek() {
        if next_char == '=' {
            comparador.push(next_char);
            caracteres.next();
        }
    }
    tokens.push(Token::Comparador(comparador));
}
```
Se o caractere atual for '>', '<' ou '!', o código verifica o próximo caractere. Se o próximo caractere for '=', ele adiciona o token Comparador(">=", "<=" ou "!=") ao vetor de tokens. Caso contrário, ele adiciona o token Comparador(">" ou "<") ao vetor de tokens.

```rust
'0'..='9' | '.' => {
    let mut numero = String::new();
    while let Some(c) = caracteres.peek() {
        if c.is_digit(10) || *c == '.' {
            numero.push(*c);
            caracteres.next();
        } else {
            break;
        }
    }
    tokens.push(Token::Numero(numero.parse().unwrap()));
}
```
Se o caractere atual for um dígito (0-9) ou um ponto, o código cria uma string vazia chamada `numero` e faz um loop para percorrer os dígitos do número. Se o caractere atual for um dígito ou um ponto, ele adiciona o caractere à string `numero` e avança para o próximo caractere. Caso contrário, ele adiciona o token `Numero(numero.parse().unwrap())` ao vetor de tokens.

```rust
'+' | '-' | '*' | '/' => {
    tokens.push(Token::Operador(caractere));
    caracteres.next();
}
```
Se o caractere atual for um operador aritmético (+, -, *, /), o código adiciona o token `Operador(caractere)` ao vetor de tokens e avança para o próximo caractere.

```rust
'(' | ')' => {
    tokens.push(Token::Parentese(caractere));
    caracteres.next();
}
```
Se o caractere atual for um parêntese de abertura ou fechamento (, ), o código adiciona o token `Parentese(caractere)` ao vetor de tokens e avança para o próximo caractere.

```rust
' ' | '\t' | '\n' => {
    caracteres.next();
}
```
Se o caractere atual for um espaço, tabulação ou quebra de linha, o código ignora e avança para o próximo caractere.

```rust
_ => {
    eprintln!("\x1b[31m    Caractere inválido: '{}' de \"{}\". Tente novamente.\x1b[0m", caractere, input);
    tokens.clear();
    break;
}
```
Se o caractere atual não corresponder a nenhum dos casos acima (`_`), o código exibe uma mensagem de erro em vermelho e limpa o vetor de tokens.

```rust
if !tokens.is_empty() {
        tokens.push(Token::FIM);
    }
    tokens
```
Ao finalizar loop, o código verifica se o vetor de tokens não está vazio devido a um erro. Se não estiver vazio, ele adiciona o token `FIM` ao vetor de tokens e retorna o vetor de tokens.







