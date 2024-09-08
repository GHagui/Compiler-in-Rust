mod lexer;
mod parser;

fn test_lexer(input: &str){
    println!("  {}", input);
    let tokens = lexer::lex(input);
    println!("\nTokens:");
    println!("  {:?}\n\n", tokens);
}

fn main(){
    println!("Entrada de usuário");
    test_lexer("Abre parênteses 1 mais 2 vezes 3 Fecha parênteses dividido por 4");
}
