
mod lexer;
use crate::lexer::Lexer;

fn main() {
    let mut l: Lexer = Lexer::new(&String::from(r#"\\"#));
    let tokens = l.scan();
    println!("{:?}", tokens);
}
