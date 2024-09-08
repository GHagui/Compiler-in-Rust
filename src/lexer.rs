#[derive(Debug, PartialEq)]
pub enum Token {
    INICIO,
    Variavel(String),
    Numero(f64),
    Operador(char),
    Parentese(char),
    FIM,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    tokens.push(Token::INICIO);

    for palavra in input.split_whitespace() {
        match palavra.to_lowercase().as_str() {
            "abre" => tokens.push(Token::Parentese('(')),
            "fecha" => tokens.push(Token::Parentese(')')),
            "mais" => tokens.push(Token::Operador('+')),
            "menos" => tokens.push(Token::Operador('-')),
            "vezes" => tokens.push(Token::Operador('*')),
            "dividido" => tokens.push(Token::Operador('/')),
            "por" => (),
            "parênteses" => (),
            _ => {
                if let Ok(numero) = palavra.parse() {
                    tokens.push(Token::Numero(numero));
                } else {
                    tokens.push(Token::Variavel(palavra.to_string()));
                }
            }
        }
    }
    tokens.push(Token::FIM);
    tokens
}