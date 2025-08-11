use std::str::Chars;

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
    pub fn next_token(&mut self) -> Token {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_chars() {
        let mut lexer = Lexer::new("a|b* (c|d)");
        assert_eq!(lexer.next_token(), (Token::TK_CHAR('a')));
        assert_eq!(lexer.next_token(), (Token::TK_OR));
        assert_eq!(lexer.next_token(), (Token::TK_CHAR('b')));
        assert_eq!(lexer.next_token(), (Token::TK_STAR));
        assert_eq!(lexer.next_token(), (Token::TK_LPAREN));
        assert_eq!(lexer.next_token(), (Token::TK_CHAR('c')));
        assert_eq!(lexer.next_token(), (Token::TK_OR));
        assert_eq!(lexer.next_token(), (Token::TK_CHAR('d')));
        assert_eq!(lexer.next_token(), (Token::TK_RPAREN));
        assert_eq!(lexer.next_token(), (Token::TK_EPS));
    }

    #[test]
    fn scan_escape() {
        let mut lexer = Lexer::new("\\a|\\b* (c|d)");
        assert_eq!(lexer.next_token(), (Token::TK_CHAR('a')));
        assert_eq!(lexer.next_token(), (Token::TK_OR));
        assert_eq!(lexer.next_token(), (Token::TK_CHAR('b')));
        assert_eq!(lexer.next_token(), (Token::TK_STAR));
        assert_eq!(lexer.next_token(), (Token::TK_LPAREN));
        assert_eq!(lexer.next_token(), (Token::TK_CHAR('c')));
        assert_eq!(lexer.next_token(), (Token::TK_OR));
        assert_eq!(lexer.next_token(), (Token::TK_CHAR('d')));
        assert_eq!(lexer.next_token(), (Token::TK_RPAREN));
        assert_eq!(lexer.next_token(), (Token::TK_EPS));
    }

    #[test]
    fn scan_whitespace() {
        let mut lexer = Lexer::new("a\t \n| b* (c | \td)");
        assert_eq!(lexer.next_token(), (Token::TK_CHAR('a')));
        assert_eq!(lexer.next_token(), (Token::TK_OR));
        assert_eq!(lexer.next_token(), (Token::TK_CHAR('b')));
        assert_eq!(lexer.next_token(), (Token::TK_STAR));
        assert_eq!(lexer.next_token(), (Token::TK_LPAREN));
        assert_eq!(lexer.next_token(), (Token::TK_CHAR('c')));
        assert_eq!(lexer.next_token(), (Token::TK_OR));
        assert_eq!(lexer.next_token(), (Token::TK_CHAR('d')));
        assert_eq!(lexer.next_token(), (Token::TK_RPAREN));
        assert_eq!(lexer.next_token(), (Token::TK_EPS));
    }
}