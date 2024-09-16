#[cfg(test)]
mod tests {
    use compiler_in_rust::parser::{Expr};
    use compiler_in_rust::parser::{avaliar};

    #[test]
    fn test_numero_simple() {
        let expr = Expr::Numero(5.0);
        assert_eq!(avaliar(&expr), 5.0);
    }

    #[test]
    fn test_soma() {
        let expr = Expr::Operacao(Box::new(Expr::Numero(3.0)), '+', Box::new(Expr::Numero(2.0)));
        assert_eq!(avaliar(&expr), 5.0);
    }

    #[test]
    fn test_operadores() {
        let expr = Expr::Operacao(
            Box::new(Expr::Operacao(Box::new(Expr::Numero(3.0)), '-', Box::new(Expr::Numero(2.0)))),
            '*',
            Box::new(Expr::Numero(4.0)),
        );
        assert_eq!(avaliar(&expr), 4.0);
    }

    #[test]
    fn test_comparacao_igual() {
        let expr = Expr::Comparacao(Box::new(Expr::Numero(3.0)), "==".to_string(), Box::new(Expr::Numero(3.0)));
        assert_eq!(avaliar(&expr), 1.0);
    }

    #[test]
    fn test_comparacao_igual_false() {
        let expr = Expr::Comparacao(Box::new(Expr::Numero(3.0)), "==".to_string(), Box::new(Expr::Numero(4.0)));
        assert_eq!(avaliar(&expr), 0.0);
    }

    #[test]
    fn test_comparacao_nao_igual_equal() {
        let expr = Expr::Comparacao(Box::new(Expr::Numero(3.0)), "!=".to_string(), Box::new(Expr::Numero(4.0)));
        assert_eq!(avaliar(&expr), 1.0);
    }

    #[test]
    fn test_comparacao_nao_igual_equal_false() {
        let expr = Expr::Comparacao(Box::new(Expr::Numero(3.0)), "!=".to_string(), Box::new(Expr::Numero(3.0)));
        assert_eq!(avaliar(&expr), 0.0);
    }

    #[test]
    fn test_comparacao_maior() {
        let expr = Expr::Comparacao(Box::new(Expr::Numero(5.0)), ">".to_string(), Box::new(Expr::Numero(3.0)));
        assert_eq!(avaliar(&expr), 1.0);
    }

    #[test]
    fn test_comparacao_maior_false() {
        let expr = Expr::Comparacao(Box::new(Expr::Numero(5.0)), ">".to_string(), Box::new(Expr::Numero(6.0)));
        assert_eq!(avaliar(&expr), 0.0);
    }


    #[test]
    fn test_comparacao_menor() {
        let expr = Expr::Comparacao(Box::new(Expr::Numero(2.0)), "<".to_string(), Box::new(Expr::Numero(3.0)));
        assert_eq!(avaliar(&expr), 1.0);
    }

    #[test]
    fn test_comparacao_menor_false() {
        let expr = Expr::Comparacao(Box::new(Expr::Numero(4.0)), "<".to_string(), Box::new(Expr::Numero(3.0)));
        assert_eq!(avaliar(&expr), 0.0);
    }

    #[test]
    fn test_comparacao_maior_ou_igual() {
        let expr = Expr::Comparacao(Box::new(Expr::Numero(3.0)), ">=".to_string(), Box::new(Expr::Numero(3.0)));
        assert_eq!(avaliar(&expr), 1.0);
    }

    #[test]
    fn test_comparacao_maior_ou_igual_false() {
        let expr = Expr::Comparacao(Box::new(Expr::Numero(3.0)), ">=".to_string(), Box::new(Expr::Numero(6.0)));
        assert_eq!(avaliar(&expr), 0.0);
    }

    #[test]
    fn test_comparacao_menor_ou_igual() {
        let expr = Expr::Comparacao(Box::new(Expr::Numero(2.0)), "<=".to_string(), Box::new(Expr::Numero(3.0)));
        assert_eq!(avaliar(&expr), 1.0);
    }

    #[test]
    fn test_comparacao_menor_ou_igual_false() {
        let expr = Expr::Comparacao(Box::new(Expr::Numero(4.0)), "<=".to_string(), Box::new(Expr::Numero(3.0)));
        assert_eq!(avaliar(&expr), 0.0);
    }

    #[test]
    fn test_numero_expr() {
        let expr = Expr::Numero(3.14);
        let debug_output = format!("{:?}", expr);
        assert_eq!(debug_output, "Numero(3.14)");
    }

    #[test]
    fn test_operadores_expr() {
        let expr1 = Expr::Numero(1.0);
        let expr2 = Expr::Numero(2.0);
        let op = Expr::Operacao(Box::new(expr1), '+', Box::new(expr2));
        let debug_output = format!("{:?}", op);
        assert_eq!(debug_output, "Operacao(Numero(1.0), '+', Numero(2.0))");
    }

    #[test]
    fn test_comparacao_expr() {
        let expr1 = Expr::Numero(5.0);
        let expr2 = Expr::Numero(10.0);
        let comp = Expr::Comparacao(Box::new(expr1), "==".to_string(), Box::new(expr2));
        let debug_output = format!("{:?}", comp);
        assert_eq!(debug_output, "Comparacao(Numero(5.0), \"==\", Numero(10.0))");
    }
}