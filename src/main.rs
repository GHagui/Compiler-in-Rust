mod lexer;
mod parser;

fn main() {
    loop {
        let mut input = String::new();

        println!("Digite uma expressão aritmética ou 'exit' para sair ou 'help' para ver as expressões suportadas:");
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
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
    }
}
