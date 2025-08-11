#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    TK_CHAR(char),
    TK_OR,
    TK_STAR,
    TK_LPAREN,
    TK_RPAREN,
    TK_EPS
}
pub struct Lexer<'a> {
    string: Chars<'a>
}

impl Lexer<'_> {
    pub fn new(string: &str) -> Lexer {
        Lexer {
            string: string.chars()
        }
    }
    pub fn next_token(&mut self) -> Option<Token> {
        use Token::*;
        let Some(ch) = self.string.next() else {
            return TK_EPS;
        };
        match ch {
            '\\' => TK_CHAR(self.string.next().unwrap_or('\\')), // escape character
            '|' | '+' => TK_OR,
            '(' => TK_LPAREN,
            ')' => TK_RPAREN,
            '*' => TK_STAR,
            ' ' | '\n' | '\t' => self.next_token(), // skip whitespace
            _ => TK_CHAR(ch)
        }
    }
}

