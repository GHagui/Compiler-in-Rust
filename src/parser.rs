use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Numero(f64),
    Operacao(Box<Expr>, char, Box<Expr>),
    Comparacao(Box<Expr>, String, Box<Expr>),
}

pub fn criar_operacao(expr: Expr, token: &Token, rhs: Expr) -> Expr {
    Expr::Operacao(
        Box::new(expr),
        if let Token::Operador(op) = token { *op } else { unreachable!() },
        Box::new(rhs),
    )
}

pub fn parse(tokens: &[Token]) -> Result<Expr, String> {
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

fn parse_comparacao(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    let mut expr = parse_expressao(tokens, pos)?;

    if let Some(Token::Comparador(op)) = tokens.get(*pos) {
        *pos += 1;
        let rhs = parse_expressao(tokens, pos)?;
        expr = Expr::Comparacao(Box::new(expr), op.clone(), Box::new(rhs));
    }

    Ok(expr)
}

fn parse_expressao(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    let mut expr = parse_termo(tokens, pos)?;

    while let Some(token) = tokens.get(*pos) {
        match token {
            Token::Operador('+') | Token::Operador('-') => {
                *pos += 1;
                let rhs = parse_termo(tokens, pos)?;
                expr = criar_operacao(expr, token, rhs);
            }
            _ => break,
        }
    }

    Ok(expr)
}

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

pub fn avaliar(expr: &Expr) -> f64 {
    match expr {
        Expr::Numero(n) => *n,
        Expr::Operacao(lhs, '+', rhs) => avaliar(lhs) + avaliar(rhs),
        Expr::Operacao(lhs, '-', rhs) => avaliar(lhs) - avaliar(rhs),
        Expr::Operacao(lhs, '*', rhs) => avaliar(lhs) * avaliar(rhs),
        Expr::Operacao(lhs, '/', rhs) => avaliar(lhs) / avaliar(rhs),
        Expr::Comparacao(lhs, op, rhs) => match op.as_str() {
            "==" => {
                if (avaliar(lhs) - avaliar(rhs)).abs() < f64::EPSILON {
                    1.0
                } else {
                    0.0
                }
            }
            "!=" => {
                if (avaliar(lhs) - avaliar(rhs)).abs() >= f64::EPSILON {
                    1.0
                }
                else {
                    0.0
                }
            }
            ">" => {
                if avaliar(lhs) > avaliar(rhs) {
                    1.0
                }
                else {
                    0.0
                }
            }
            "<" => {
                if avaliar(lhs) < avaliar(rhs) {
                    1.0
                }
                else {
                    0.0
                }
            }
            ">=" => {
                if avaliar(lhs) >= avaliar(rhs) {
                    1.0
                }
                else {
                    0.0
                }
            }
            "<=" => {
                if avaliar(lhs) <= avaliar(rhs) {
                    1.0
                }
                else {
                    0.0
                }
            }
            _ => panic!("Erro: Operador de comparação inválido"),
        },
        _ => panic!("Erro: Expressão inválida"),
    }
}
