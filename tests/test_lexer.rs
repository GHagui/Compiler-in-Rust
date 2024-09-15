extern crate compiler_in_rust;
#[cfg(test)]
mod tests {
    use compiler_in_rust::lexer;
    use compiler_in_rust::lexer::Token;

    #[test]
    fn test_lexer_declaracao_variavel() {
        let input = "a = 13";
        let tokens = lexer::lex(input);
        let tokens_expectativas = vec![
            Token::INICIO,
            Token::Variavel('a'),
            Token::Operador('='),
            Token::Numero(13.0),
            Token::FIM
        ];
        assert_eq!(tokens, tokens_expectativas);
    }

    #[test]
    fn test_lexer_soma() {
        let input = "13 + 2";
        let tokens = lexer::lex(input);
        let tokens_expectativas = vec![
            Token::INICIO,
            Token::Numero(13.0),
            Token::Operador('+'),
            Token::Numero(2.0),
            Token::FIM
        ];
        assert_eq!(tokens, tokens_expectativas);
    }

    #[test]
    fn test_lexer_subtracao() {
        let input = "13 - 2";
        let tokens = lexer::lex(input);
        let tokens_expectativas = vec![
            Token::INICIO,
            Token::Numero(13.0),
            Token::Operador('-'),
            Token::Numero(2.0),
            Token::FIM
        ];
        assert_eq!(tokens, tokens_expectativas);
    }

    #[test]
    fn test_lexer_multiplicacao() {
        let input = "13 * 2";
        let tokens = lexer::lex(input);
        let tokens_expectativas = vec![
            Token::INICIO,
            Token::Numero(13.0),
            Token::Operador('*'),
            Token::Numero(2.0),
            Token::FIM
        ];
        assert_eq!(tokens, tokens_expectativas);
    }

    #[test]
    fn test_lexer_divisao() {
        let input = "13 / 2";
        let tokens = lexer::lex(input);
        let tokens_expectativas = vec![
            Token::INICIO,
            Token::Numero(13.0),
            Token::Operador('/'),
            Token::Numero(2.0),
            Token::FIM
        ];
        assert_eq!(tokens, tokens_expectativas);
    }

    #[test]
    fn test_lexer_expressao_parenteses() {
        let input = "(13 + 2) * 3";
        let tokens = lexer::lex(input);
        let tokens_expectativas = vec![
            Token::INICIO,
            Token::Parentese('('),
            Token::Numero(13.0),
            Token::Operador('+'),
            Token::Numero(2.0),
            Token::Parentese(')'),
            Token::Operador('*'),
            Token::Numero(3.0),
            Token::FIM
        ];
        assert_eq!(tokens, tokens_expectativas);
    }

    #[test]
    fn test_lexer_numero_ponto_flutuante() {
        let input = "13.5 + 2.5";
        let tokens = lexer::lex(input);
        let tokens_expectativas = vec![
            Token::INICIO,
            Token::Numero(13.5),
            Token::Operador('+'),
            Token::Numero(2.5),
            Token::FIM
        ];
        assert_eq!(tokens, tokens_expectativas);
    }

    #[test]
    fn test_lexer_comparacao_igual() {
        let input = "13 == 13";
        let tokens = lexer::lex(input);
        let tokens_expectativas = vec![
            Token::INICIO,
            Token::Numero(13.0),
            Token::Comparador("==".to_string()),
            Token::Numero(13.0),
            Token::FIM
        ];
        assert_eq!(tokens, tokens_expectativas);
    }

    #[test]
    fn test_lexer_comparacao_diferente() {
        let input = "13 != 13";
        let tokens = lexer::lex(input);
        let tokens_expectativas = vec![
            Token::INICIO,
            Token::Numero(13.0),
            Token::Comparador("!=".to_string()),
            Token::Numero(13.0),
            Token::FIM
        ];
        assert_eq!(tokens, tokens_expectativas);
    }

    #[test]
    fn test_lexer_comparacao_maior() {
        let input = "13 > 13";
        let tokens = lexer::lex(input);
        let tokens_expectativas = vec![
            Token::INICIO,
            Token::Numero(13.0),
            Token::Comparador(">".to_string()),
            Token::Numero(13.0),
            Token::FIM
        ];
        assert_eq!(tokens, tokens_expectativas);
    }

    #[test]
    fn test_lexer_comparacao_menor() {
        let input = "13 < 13";
        let tokens = lexer::lex(input);
        let tokens_expectativas = vec![
            Token::INICIO,
            Token::Numero(13.0),
            Token::Comparador("<".to_string()),
            Token::Numero(13.0),
            Token::FIM
        ];
        assert_eq!(tokens, tokens_expectativas);
    }
}