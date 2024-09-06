mod lexer;
mod parser;

fn test_lexer(input: &str){
    println!("  {}", input);
    let tokens = lexer::lex(input);
    println!("  {:?}\n\n", tokens);
}

fn main(){
    println!("Testando declaração de variável:");
    test_lexer("a = 13");

    println!("Testando soma:");
    test_lexer("13 + 2");

    println!("Testando subtração:");
    test_lexer("13 - 2");

    println!("Testando multiplicação:");
    test_lexer("13 * 2");

    println!("Testando divisão:");
    test_lexer("13 / 2");

    println!("Testando expressão com parênteses:");
    test_lexer("(13 + 2) * 3");

    println!("Testando número com ponto flutuante:");
    test_lexer("13.5 + 2.5");

    println!("Testando erro de caractere inválido:");
    test_lexer("13 + 2 # 3");
}
