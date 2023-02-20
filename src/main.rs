
mod lexer;
mod parser;

use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let mut l: Lexer = Lexer::new(r#"(james)*|(jake)"#);
    let tokens = l.scan();
    println!("{:?}", tokens);

    let mut p: Parser = Parser::new(tokens);
    p.parse();
}
