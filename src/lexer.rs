#[derive(Debug, PartialEq)]
pub enum Token {
    INICIO,
    Variavel(char),
    Numero(f64),
    Operador(char),
    Comparador(char),
    Parentese(char),
    FIM,
}
pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut caracteres = input.chars().peekable();
    tokens.push(Token::INICIO);
    while let Some(&caractere) = caracteres.peek() {
        match caractere {
            'a'..='z' | 'A'..='Z' | '=' => {
                caracteres.next();

                if caractere == '=' {
                    if let Some(&next_char) = caracteres.peek() {
                        if next_char == '=' {
                            tokens.push(Token::Comparador(next_char));
                            caracteres.next();
                        } else {
                            tokens.push(Token::Operador(caractere));
                        }
                    }
                } else {
                    tokens.push(Token::Variavel(caractere));
                }
            }
            '>' | '<' | '!' => {
                caracteres.next();

                if let Some(&next_char) = caracteres.peek() {
                    if next_char == '=' {
                        tokens.push(Token::Comparador(caractere));
                        tokens.push(Token::Comparador(next_char));
                        caracteres.next();
                    } else {
                        tokens.push(Token::Comparador(caractere));
                    }
                }
            }
            '0'..='9' | '.' => {
                let mut numero = String::new();
                while let Some('0'..='9' | '.') = caracteres.peek() {
                    numero.push(caracteres.next().unwrap());
                }
                tokens.push(Token::Numero(numero.parse().unwrap()));
            }
            '+' | '-' | '*' | '/' => {
                tokens.push(Token::Operador(caractere));
                caracteres.next();
            }
            '(' | ')' => {
                tokens.push(Token::Parentese(caractere));
                caracteres.next();
            }
            ' ' => {
                caracteres.next();
            }
            _ => {
                panic!("Caractere inválido: {} de \" {} \". \nTente novamente", caractere, input);
            }
        }
    }
    tokens.push(Token::FIM);
    tokens
}
