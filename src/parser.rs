
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

/**
 * Barebones Regex CFG
 *
 * REGEX -> <TERMINAL> or <REGEX><REGEX> or <REGEX>|<REGEX> or <REGEX>* or <OpenParen><REGEX><CloseParen>
 * TERMINAL -> <EmptyString> | <EmptySet> | <CHAR>
 * CHAR  -> { c | c in ASCII256, c != whitespace }
 *
 *
 * Note: Consider defining some kind of WORD symbol to implicitly concatenate adjacent chars.
 *       Example: The regex `jake` would be a terminal symbol rather than a concatenation of
 *       four CHAR regular expressions.
 *
 */

 /**
  * Operator Precedence
  *
  * Our regular expression language supports the three fundamental operations,
  * in order of precedence:
  *     1. Kleene star    --> R*
  *     2. Concatentation --> RS
  *     3. Alternation    --> R|S
  *
  * Parentheses may be used to group expressions and override precedence rules.
  *
  * Examples:
  *
  *     RS*   ==  R(S*)    (Kleene has higher precedence than concatenation.)
  *
  *     RS|A  ==  (RS)|A   (Concatenation has higher precendence than alternation.)
  */
