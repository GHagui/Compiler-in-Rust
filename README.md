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
A linguagem Rust foi escolhida por ser moderna, segura e eficiente. Ao contrário de Java, Rust não utiliza um garbage collector, mas assegura a segurança de memória através do conceito de ownership. De acordo com a documentação oficial:

> "Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running." ([Rust Book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html))

Isso ajuda a evitar problemas comuns em C e C++, oferecendo desempenho semelhante a essas linguagens. Além disso, a [Microsoft](https://opensource.microsoft.com/blog/2021/02/08/microsoft-joins-rust-foundation/) e a [Google](https://opensource.googleblog.com/2021/02/google-joins-rust-foundation.html) estão investindo em Rust para o desenvolvimento de drivers, sistemas internos e até mesmo no Windows e Android, o que evidencia ainda mais o potencial da linguagem.

## Dependências e ferramentas
Este projeto foi desenvolvido sem o uso de bibliotecas externas, apenas com as bibliotecas padrão do Rust. Nem ferramenta como ANTLR (Não tem suporte para Rust) ou Pest foi utilizada, pois o projeto é simples e não requer uma análise complexa da gramática já que é a calculadora básica.

## Etapas do compilador
- Entrada de usuário (expressão matemática)
- Análise léxica
- Análise sintática
- Análise semântica
- Saída do resultado

## Explicação do código
O código foi dividido em módulos para facilitar a organização e manutenção. O módulo `lexer` é responsável por realizar a análise léxica, enquanto o módulo `parser` é responsável pela análise sintática e semântica.

No código principal, a função `main` é responsável por receber a entrada do usuário, chamar a função `lexer::lex` para realizar a análise léxica e, em seguida, chamar a função `parser::parse` para realizar a análise sintática e semântica. Para começar, o código principal exibe uma mensagem ao usuário e entra em um loop para que o usuário possa digitar várias expressões sem precisar reiniciar o programa.
### main.rs
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
No código acima, o caso `help` exibe as expressões suportadas pelo compilador. O `exit` encerra o programa. Se o usuário digite a expressão, o programa chama a função `lexer::lex(input)` para realizar a análise léxica e a função `parser::parse(tokens)` para realizar a análise sintática e semântica. Em seguida, exibe os tokens gerados, a árvore sintática e o resultado da expressão.

### lexer.rs
Ao chamar a função `lexer::lex(input)`, o código passa a expressão do usuário em formato de string e cria um vetor de tokens. Segue o código abaixo:
```rust
pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut caracteres = input.chars().peekable();
    tokens.push(Token::INICIO);
    
    // ( ... )
}
```
Em seguida, faz um push de INICIO no vetor de tokens (`tokens.push(Token::INICIO);`). Token é um enumeration que representa os tipos de tokens suportados pelo compilador. Segue o código do enumeration abaixo:
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

Em seguida o código faz um loop `while` para percorrer os caracteres da expressão do usuário e identificar os tokens correspondentes.
```rust
while let Some(&caractere) = caracteres.peek() {
        match caractere {
        //( ... )
        }
}
```
Essa parte do código acima `match` é parecido com um switch case, onde ele verifica o caractere atual e faz a ação correspondente. Segue o código que verifica o caractere atual:
```rust
'=' => {
    caracteres.next();
    if let Some(&next_char) = caracteres.peek() {
        if next_char == '=' {
        tokens.push(Token::Comparador("==".to_string()));
        caracteres.next();
        }
        else {
        eprintln!("\x1b[31m    Caractere inválido: '{}' de \"{}\". Tente novamente.\x1b[0m", caractere, input);
        tokens.clear();
        break;
        }
    }
}
```
No código acima, se o caractere atual for '=', o código verifica o próximo caractere. Se o próximo caractere for '=', ele adiciona o token Comparador("==") ao vetor de tokens. Caso contrário, ele exibe uma mensagem de erro em vermelho e limpa o vetor de tokens, assim evita que o usuário digite uma expressão como `=>` ou `=<`.

Se o caractere atual for '>', '<' ou '!', o código verifica o próximo caractere. Se o próximo caractere for '=', ele adiciona o token Comparador(">=", "<=" ou "!=") ao vetor de tokens. Caso contrário, ele adiciona o token Comparador(">" ou "<") ao vetor de tokens. Segue o código abaixo:
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
Se o caractere atual for um dígito (0-9) ou um ponto, o código cria uma string vazia chamada `numero` e faz um loop para percorrer os dígitos do número. Se o caractere atual for um dígito ou um ponto, ele adiciona o caractere à string `numero` e avança para o próximo caractere. Caso contrário, ele adiciona o token `Numero(numero.parse().unwrap())` ao vetor de tokens.

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
Se o caractere atual for um operador aritmético (+, -, *, /), o código adiciona o token `Operador(caractere)` ao vetor de tokens e avança para o próximo caractere.

```rust
'+' | '-' | '*' | '/' => {
    tokens.push(Token::Operador(caractere));
    caracteres.next();
}
```
Se o caractere atual for um parêntese de abertura ou fechamento (, ), o código adiciona o token `Parentese(caractere)` ao vetor de tokens e avança para o próximo caractere.

```rust
'(' | ')' => {
    tokens.push(Token::Parentese(caractere));
    caracteres.next();
}
```
Se o caractere atual for um espaço, tabulação ou quebra de linha, o código ignora e avança para o próximo caractere.

```rust
' ' | '\t' | '\n' => {
    caracteres.next();
}
```
Se o caractere atual não corresponder a nenhum dos casos acima (`_`), o código exibe uma mensagem de erro em vermelho e limpa o vetor de tokens.

```rust
_ => {
    eprintln!("\x1b[31m    Caractere inválido: '{}' de \"{}\". Tente novamente.\x1b[0m", caractere, input);
    tokens.clear();
    break;
}
```
Ao finalizar loop, o código verifica se o vetor de tokens não está vazio devido a um erro. Se não estiver vazio, ele adiciona o token `FIM` ao vetor de tokens e retorna o vetor de tokens.

```rust
if !tokens.is_empty() {
        tokens.push(Token::FIM);
    }
    tokens
```

Depois de chamar a função `lexer::lex(input)`, o código chama a função `parser::parse(&tokens)` para realizar a análise sintática e semântica. No código `parser.rs`, a função `parse` recebe um vetor de tokens e retorna um resultado em integer.

### parser.rs

A enumeração `Expr` é criada e usada para representar a árvore sintática da expressão. A enumeração possui os seguintes tipos de expressões:
- `Numero(f64)`: Um número inteiro ou decimal em formato de ponto flutuante.
- `Operacao(Box<Expr>, char, Box<Expr>)`: Box<Expr> é um ponteiro para uma expressão. A char é um operador aritmético (+, -, *, /).
- `Comparacao(Box<Expr>, String, Box<Expr>)`: Box<Expr> é um ponteiro para uma expressão. A String é um operador de comparação (==, !=, >, <, >=, <=).
```rust
pub enum Expr {
    Numero(f64),
    Operacao(Box<Expr>, char, Box<Expr>),
    Comparacao(Box<Expr>, String, Box<Expr>),
}
```

Para iniciar por função `parse`, o código verifica se o vetor de tokens não está vazio. Se estiver vazio, ele retorna um erro. Depois disso, a variável `pos` é inicializada com 0 que representa a posição atual no vetor de tokens. Em seguida, o código verifica se o primeiro token é `INICIO`. Se não for, ele retorna um erro. Caso contrário, ele avança para o próximo token e chama a função `parse_comparacao(tokens, &mut pos)` para realizar a análise sintática e semântica da comparação. Se a função `parse_comparacao` retornar um erro, ele retorna o erro. Caso contrário, ele verifica se o último token é `FIM`. Se não for, ele retorna um erro. Caso contrário, ele retorna a expressão.

```rust
pub fn parse(tokens: &[Token]) -> Result<Expr, String> {
    if tokens.is_empty() {
        return Err("\x1b[31m    Erro: Expressão inválida na análise léxica. Digite 'help' para ver as expressões suportadas.\x1b[0m".to_string());
    }
    let mut pos = 0;
    if let Some(Token::INICIO) = tokens.get(pos) {
        pos += 1;
        let expr = parse_comparacao(tokens, &mut pos)?;
        if let Some(Token::FIM) = tokens.last() {
            Ok(expr)
        } else {
            Err("Erro: Esperado FIM".to_string())
        }
    } else {
        Err("Erro: Esperado INICIO".to_string())
    }
}
```

A função `parse_comparacao` recebe um vetor de tokens e uma variável `pos`. A função chama a função `parse_soma` para realizar a análise sintática e semântica da soma. Se o próximo token for `Token::Comparador`, avança `pos`, chama `parse_expressao` novamente e cria um Expr::Comparacao que representa a comparação entre as duas expressões.
```rust
fn parse_comparacao(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    let mut expr = parse_expressao(tokens, pos)?;

    if let Some(Token::Comparador(op)) = tokens.get(*pos) {
        *pos += 1;
        let rhs = parse_expressao(tokens, pos)?;
        expr = Expr::Comparacao(Box::new(expr), op.clone(), Box::new(rhs));
    }

    Ok(expr)
}
```
A função `parse_termo` trata operações de multiplicação e divisão, que têm maior precedência. Ela inicia chamando `parse_fator` para analisar o primeiro fator. Em seguida, em um loop, verifica se o próximo token é um operador `*` ou `/`. Se for, avança `pos`, analisa o próximo fator e cria uma operação com `criar_operacao`.
```rust
fn parse_termo(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    let mut expr = parse_fator(tokens, pos)?;

    while let Some(token) = tokens.get(*pos) {
        match token {
            Token::Operador('*') | Token::Operador('/') => {
                *pos += 1;
                let rhs = parse_fator(tokens, pos)?;
                expr = criar_operacao(expr, token, rhs);
            }
            _ => break,
        }
    }

    Ok(expr)
}
```
A função `parse_fator` trata números e expressões entre parênteses. Se o próximo token for um número, cria uma expressão `Expr::Numero`. Se for um parêntese de abertura, avança `pos`, chama `parse_comparacao` e verifica se o próximo token é um parêntese de fechamento. Se não for, retorna um erro.
```rust
fn parse_fator(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    match tokens.get(*pos) {
        Some(Token::Numero(n)) => {
            *pos += 1;
            Ok(Expr::Numero(*n))
        }
        Some(Token::Parentese('(')) => {
            *pos += 1;
            let expr = parse_expressao(tokens, pos)?;
            if let Some(Token::Parentese(')')) = tokens.get(*pos) {
                *pos += 1;
                Ok(expr)
            } else {
                Err("Erro: Esperado ')'".to_string())
            }
        }
        _ => Err("Erro: Fator inválido".to_string()),
    }
}
```
A função `criar_operacao` recebe uma expressão, um token e rhs e cria uma expressão `Expr::Operacao` que representa a operação entre as duas expressões.
```rust
pub fn criar_operacao(expr: Expr, token: &Token, rhs: Expr) -> Expr {
    Expr::Operacao(
        Box::new(expr),
        if let Token::Operador(op) = token { *op } else { unreachable!() },
        Box::new(rhs),
    )
}
```
A função `avaliar` recebe uma expressão e avalia o resultado da expressão. Se a expressão for um número, retorna o número. Se a expressão for uma operação, avalia a operação e retorna o resultado. Se a expressão for uma comparação, avalia a comparação e retorna 1.0 se a comparação for verdadeira e 0.0 se for falsa.
```rust
pub fn avaliar(expr: &Expr) -> f64 {
    match expr {
        Expr::Numero(n) => *n,
        Expr::Operacao(lhs, op, rhs) => {
            let lhs_val = avaliar(lhs);
            let rhs_val = avaliar(rhs);
            match op {
                '+' => lhs_val + rhs_val,
                '-' => lhs_val - rhs_val,
                '*' => lhs_val * rhs_val,
                '/' => lhs_val / rhs_val,
                _ => {
                    eprintln!("\x1b[31mErro: Operador aritmético inválido\x1b[0m");
                    0.0
                }
            }
        }
        Expr::Comparacao(lhs, op, rhs) => {
            let lhs_val = avaliar(lhs);
            let rhs_val = avaliar(rhs);
            match op.as_str() {
                "==" => if (lhs_val - rhs_val).abs() < f64::EPSILON { 1.0 } else { 0.0 },
                "!=" => if (lhs_val - rhs_val).abs() >= f64::EPSILON { 1.0 } else { 0.0 },
                ">"  => if lhs_val > rhs_val { 1.0 } else { 0.0 },
                "<"  => if lhs_val < rhs_val { 1.0 } else { 0.0 },
                ">=" => if lhs_val >= rhs_val { 1.0 } else { 0.0 },
                "<=" => if lhs_val <= rhs_val { 1.0 } else { 0.0 },
                _ => {
                    eprintln!("\x1b[31mErro: Operador de comparação inválido\x1b[0m");
                    0.0
                }
            }
        }
    }
}
```
## Fluxo de Análise Sintática
1. `parse`: Verifica tokens iniciais e finais, inicia análise com parse_comparacao.
2. `parse_comparacao`: Analisa expressões com comparadores.
3. `parse_expressao`: Analisa adições e subtrações.
4. `parse_termo`: Analisa multiplicações e divisões.
5. `parse_fator`: Analisa números e expressões entre parênteses.

**Referências:**

- RUST BOOK. Ownership. Disponível em: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html. Acesso em: 05 set. 2024.
- RUST DOCUMENTATION. Disponível em: https://doc.rust-lang.org/book/. Acesso em: 05 set. 2024.
- GOOGLE. Google joins Rust Foundation. Disponível em: https://opensource.googleblog.com/2021/02/google-joins-rust-foundation.html. Acesso em: 16 set. 2024. 
- MICROSOFT. Microsoft joins Rust Foundation. Disponível em: https://opensource.microsoft.com/blog/2021/02/08/microsoft-joins-rust-foundation/. Acesso em: 16 set. 2024.







