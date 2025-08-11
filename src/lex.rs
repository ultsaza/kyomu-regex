use std::str::Chars;
use std::fmt::Display;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    TkChar(char),
    TkOr,
    TkStar,
    TkLparen,
    TkRparen,
    TkEps
}

pub struct Lexer<'a> {
    string: Chars<'a>
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        let str = match self {
            TkChar(_) => "Char",
            TkOr => "|",
            TkStar => "*",
            TkLparen => "(",
            TkRparen => ")",
            TkEps => "ε",
        };
        write!(f, "{}", str)
    }
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
            return TkEps;
        };
        match ch {
            '\\' => TkChar(self.string.next().unwrap_or('\\')), // escape character
            '|' | '+' => TkOr,
            '(' => TkLparen,
            ')' => TkRparen,
            '*' => TkStar,
            ' ' | '\n' | '\t' => self.next_token(), // skip whitespace
            _ => TkChar(ch)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_chars() {
        let mut lexer = Lexer::new("a|b* (c|d)");
        assert_eq!(lexer.next_token(), (Token::TkChar('a')));
        assert_eq!(lexer.next_token(), (Token::TkOr));
        assert_eq!(lexer.next_token(), (Token::TkChar('b')));
        assert_eq!(lexer.next_token(), (Token::TkStar));
        assert_eq!(lexer.next_token(), (Token::TkLparen));
        assert_eq!(lexer.next_token(), (Token::TkChar('c')));
        assert_eq!(lexer.next_token(), (Token::TkOr));
        assert_eq!(lexer.next_token(), (Token::TkChar('d')));
        assert_eq!(lexer.next_token(), (Token::TkRparen));
        assert_eq!(lexer.next_token(), (Token::TkEps));
    }

    #[test]
    fn scan_escape() {
        let mut lexer = Lexer::new("\\a|\\b* (c|d)");
        assert_eq!(lexer.next_token(), (Token::TkChar('a')));
        assert_eq!(lexer.next_token(), (Token::TkOr));
        assert_eq!(lexer.next_token(), (Token::TkChar('b')));
        assert_eq!(lexer.next_token(), (Token::TkStar));
        assert_eq!(lexer.next_token(), (Token::TkLparen));
        assert_eq!(lexer.next_token(), (Token::TkChar('c')));
        assert_eq!(lexer.next_token(), (Token::TkOr));
        assert_eq!(lexer.next_token(), (Token::TkChar('d')));
        assert_eq!(lexer.next_token(), (Token::TkRparen));
        assert_eq!(lexer.next_token(), (Token::TkEps));
    }

    #[test]
    fn scan_whitespace() {
        let mut lexer = Lexer::new("a\t \n| b* (c | \td)");
        assert_eq!(lexer.next_token(), (Token::TkChar('a')));
        assert_eq!(lexer.next_token(), (Token::TkOr));
        assert_eq!(lexer.next_token(), (Token::TkChar('b')));
        assert_eq!(lexer.next_token(), (Token::TkStar));
        assert_eq!(lexer.next_token(), (Token::TkLparen));
        assert_eq!(lexer.next_token(), (Token::TkChar('c')));
        assert_eq!(lexer.next_token(), (Token::TkOr));
        assert_eq!(lexer.next_token(), (Token::TkChar('d')));
        assert_eq!(lexer.next_token(), (Token::TkRparen));
        assert_eq!(lexer.next_token(), (Token::TkEps));
    }
}