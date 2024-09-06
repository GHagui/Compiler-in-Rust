
// Definindo os tokens
#[derive(Debug, PartialEq)]
pub(crate) enum  Token{
    INICIO,
    Variavel(char),
    Numero(f64),
    Operador(char),
    Parentese(char),
    FIM
}

pub(crate) fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut caracteres = input.chars().peekable();
    tokens.push(Token::INICIO);
    while let Some(caractere) = caracteres.peek() {
        match caractere {
            'a'..='z' | 'A'..='Z' | '=' => {
                tokens.push(Token::Variavel(*caractere));
                caracteres.next();
            }
            '0'..='9' | '.' => {
                let mut numero = String::new();
                while let Some('0'..='9' | '.') = caracteres.peek() {
                    numero.push(caracteres.next().unwrap());
                }
                tokens.push(Token::Numero(numero.parse().unwrap()));
            }
            '+' | '-' | '*' | '/' => {
                tokens.push(Token::Operador(*caractere));
                caracteres.next();
            }
            '(' | ')' => {
                tokens.push(Token::Parentese(*caractere));
                caracteres.next();
            }
            ' ' => {
                caracteres.next();
            }
            _ => {
                panic!("Caractere inválido: {} de \" {} \"", caractere, input);
            }

        }
    }
    tokens.push(Token::FIM);
    tokens
}