
use crate::lexer::Token;

pub struct Parser {
  tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens
        }
    }

    pub fn parse(&mut self) -> () {
        todo!();
    }
}