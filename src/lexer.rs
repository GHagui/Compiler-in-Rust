#[derive(Debug, PartialEq)]
pub enum Token {
    INICIO,
    Numero(f64),
    Operador(char),
    Comparador(String),
    Parentese(char),
    FIM,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut caracteres = input.chars().peekable();
    tokens.push(Token::INICIO);

    while let Some(&caractere) = caracteres.peek() {
        match caractere {
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
            '+' | '-' | '*' | '/' => {
                tokens.push(Token::Operador(caractere));
                caracteres.next();
            }
            '(' | ')' => {
                tokens.push(Token::Parentese(caractere));
                caracteres.next();
            }
            ' ' | '\t' | '\n' => {
                caracteres.next();
            }
            _ => {
                eprintln!("\x1b[31m    Caractere inválido: '{}' de \"{}\". Tente novamente.\x1b[0m", caractere, input);
                tokens.clear();
                break;
            }
        }
    }
    if !tokens.is_empty() {
        tokens.push(Token::FIM);
    }
    tokens
}
