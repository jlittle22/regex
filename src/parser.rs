
/** Design

ABSTRACT

Parser must derive syntactic structure from a sequence of tokens. If the token
stream represents a valid regular expression, it produces an Abstract Syntax
Tree representing the expression. Otherwise, it returns an error.

SYNTAX

See the "Barebones Regex CFG" at the bottom of this file for an exact syntax
specification.

POSSIBLE APPROACHES

There are two basic parsing styles we could pursue.

1. Top-down: Matches the input stream against the productions of the CFG by
   predicting the next token at each point.

2. Bottom-up: Accumulates context from the sequence of tokens until a valid CFG
   derivation is found.


*/


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
 * This CFG is structured in such a way that enforces the operator precedence
 * described below. Parse trees generated from this CFG can be 'evaluated'
 * with a simple postorder traversal and be certain to obey precedence.
 *
 * (<REGEX> is the goal symbol.)
 *
 * <REGEX>           := <LOW_PRECEDENCE>
 * <LOW_PRECEDENCE>  := <MED_PRECEDENCE>|<MED_PRECEDENCE> or <MED_PRECEDENCE>
 * <MED_PRECEDENCE>  := <HIGH_PRECEDENCE><HIGH_PRECEDENCE> or <HIGH_PRECEDENCE>
 * <HIGH_PRECEDENCE> := <GIGA_PRECEDENCE>* or <GIGA_PRECEDENCE>
 * <GIGA_PRECEDENCE> := OpenParen<LOW_PRECEDENCE>CloseParen or <TERMINAL>
 * <TERMINAL>        := EmptyString or EmptySet or <CHAR>
 * <CHAR>            := { c | c in ASCII256, c != whitespace }
 *
 * Note: Consider defining some kind of WORD symbol to implicitly concatenate
 *       adjacent chars. E.g. the regex `jake` would be a terminal symbol rather
 *       than a concatenation of four CHAR regular expressions.
 *
 */

 /**
  * Operator Precedence
  *
  * Our regular expression language supports the three fundamental operations
  * plus grouping (in order of precedence):
  *     0. Grouping       --> (R)
  *     1. Kleene star    --> R*
  *     2. Concatentation --> RS
  *     3. Alternation    --> R|S
  *
  * Examples:
  *
  *     RS*  == R(S*)    (Kleene has higher precedence than concatenation.)
  *
  *     RS|A == (RS)|A   (Concatenation has higher precedence than alternation.)
  */
