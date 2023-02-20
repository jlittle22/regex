
#[derive(Debug, PartialEq)]
pub enum Token {
    EmptySet,       // \0
    EmptyString,    // $
    Literal(char),  // <char>
    OpenParen,      // (
    CloseParen,     // )
    Alternation,    // |
    Kleene,         // *
}

fn is_escapable_char(c: char) -> bool {
    match c {
        '\\' | '*' | '(' | ')' | '|' | '0' | '$' => true,
        _ => false
    }
}

pub struct Lexer {
    text: Vec<u8>,
    position: usize,
}

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        Lexer {
            text: text.as_bytes().to_vec(),
            position: 0,
        }
    }

    fn peek_next_byte(&self) -> Option<char> {
        if self.position + 1 >= self.text.len() {
            return None
        }

        return Some(self.text[self.position + 1] as char);
    }

    pub fn scan(&mut self) ->Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let num_bytes = self.text.len();

        while self.position < num_bytes {
            let curr_byte = self.text[self.position] as char;
            let t: Token = match curr_byte {
                '\\' => {
                    match self.peek_next_byte() {
                        None => Token::Literal(curr_byte),
                        Some(new_literal) => {
                            if is_escapable_char(new_literal) {
                                self.position += 1;
                                if new_literal == '0' {
                                    Token::EmptySet
                                } else {
                                    Token::Literal(new_literal)
                                }
                            } else {
                                Token::Literal(curr_byte)
                            }
                        },
                    }
                },
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '|' => Token::Alternation,
                '*' => Token::Kleene,
                '$' => Token::EmptyString,
                _ if curr_byte.is_whitespace() => {
                    self.position += 1;
                    continue;
                },
                _ => Token::Literal(curr_byte),
            };

            tokens.push(t);
            self.position += 1;
        }

        tokens
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty_string() {
        let mut l: Lexer = Lexer::new(r#"$"#);
        let ts = l.scan();
        assert_eq!(ts, [Token::EmptyString]);
    }

    #[test]
    fn is_escaped_empty_string() {
        let mut l: Lexer = Lexer::new(r#"\$"#);
        let ts = l.scan();
        assert_eq!(ts, [Token::Literal('$')]);
    }

    #[test]
    fn is_empty_set() {
        let mut l: Lexer = Lexer::new(r#"\0"#);
        let ts = l.scan();
        assert_eq!(ts, [Token::EmptySet]);
    }
    
    #[test]
    fn is_escaped_empty_set() {
        let mut l: Lexer = Lexer::new(r#"\\0"#);
        let ts = l.scan();
        assert_eq!(ts, [Token::Literal('\\'), Token::Literal('0')]);
    }

    #[test]
    fn is_open_paren() {
        let mut l: Lexer = Lexer::new(r#"("#);
        let ts = l.scan();
        assert_eq!(ts, [Token::OpenParen]);
    }

    #[test]
    fn is_escaped_open_paren() {
        let mut l: Lexer = Lexer::new(r#"\("#);
        let ts = l.scan();
        assert_eq!(ts, [Token::Literal('(')]);
    }

    #[test]
    fn is_close_paren() {
        let mut l: Lexer = Lexer::new(r#")"#);
        let ts = l.scan();
        assert_eq!(ts, [Token::CloseParen]);
    }

    #[test]
    fn is_escaped_close_paren() {
        let mut l: Lexer = Lexer::new(r#"\)"#);
        let ts = l.scan();
        assert_eq!(ts, [Token::Literal(')')]);
    }

    #[test]
    fn is_alternation() {
        let mut l: Lexer = Lexer::new(r#"|"#);
        let ts = l.scan();
        assert_eq!(ts, [Token::Alternation]);
    }

    #[test]
    fn is_escaped_alternation() {
        let mut l: Lexer = Lexer::new(r#"\|"#);
        let ts = l.scan();
        assert_eq!(ts, [Token::Literal('|')]);
    }

    #[test]
    fn is_kleene() {
        let mut l: Lexer = Lexer::new(r#"*"#);
        let ts = l.scan();
        assert_eq!(ts, [Token::Kleene]);
    }

    #[test]
    fn is_escaped_kleene() {
        let mut l: Lexer = Lexer::new(r#"\*"#);
        let ts = l.scan();
        assert_eq!(ts, [Token::Literal('*')]);
    }

    #[test]
    fn regex_with_trailing_backslash() {
        let mut l: Lexer = Lexer::new(r#"a*b|cd0\"#);
        let ts = l.scan();
        assert_eq!(ts, [Token::Literal('a'), 
                        Token::Kleene, 
                        Token::Literal('b'), 
                        Token::Alternation, 
                        Token::Literal('c'), 
                        Token::Literal('d'), 
                        Token::Literal('0'), 
                        Token::Literal('\\')]);
    }

    #[test]
    fn regex_empty_set_or_empty_string() {
        let mut l: Lexer = Lexer::new(r#"\0|$"#);
        let ts = l.scan();
        assert_eq!(ts, [Token::EmptySet, Token::Alternation, Token::EmptyString]);
    }

    #[test]
    fn empty_regex() {
        let mut l: Lexer = Lexer::new(r#""#);
        let ts = l.scan();
        assert_eq!(ts, []);
    }
}