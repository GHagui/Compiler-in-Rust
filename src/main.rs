mod lexer;
mod parser;
fn test_lexer(input: &str){
    println!("  {}", input);
    let tokens = lexer::lex(input);
    println!("  {:?}\n\n", tokens);
}
fn main(){
    test_lexer("14 != 13");
}
