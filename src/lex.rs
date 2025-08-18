use std::str::Chars;
use std::fmt::Display;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    TkChar(char),
    TkOr,
    TkStar,
    TkPlus,
    TkQuestion,
    TkLparen,
    TkRparen,
    TkBracket(u32, Option<u32>),  // max == none means unbounded
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
            TkPlus => "+",
            TkQuestion => "?",
            TkLparen => "(",
            TkRparen => ")",
            TkBracket {..} => "Bracket",
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
            '|' => TkOr,
            '(' => TkLparen,
            ')' => TkRparen,
            '*' => TkStar,
            '+' => TkPlus,
            '?' => TkQuestion,
            '{' => self.next_token_with_bracket(), 
            ' ' | '\n' | '\t' => self.next_token(), // skip whitespace
            _ => TkChar(ch)
        }
    }

    fn next_token_with_bracket(&mut self) -> Token {
        use Token::*;
        let mut min = 0;
        let mut max = None; // -1 indicates unbounded
        let mut is_min = true;
        while let Some(ch) = self.string.next() {
            match ch {
                '0'..='9' => {
                    let d = ch.to_digit(10).unwrap();
                    if is_min {
                        min = min * 10 + d;
                    } else {
                        max = Some(max.unwrap() * 10 + d);
                    }
                }
                ',' => {
                    if is_min { 
                        is_min = false; 
                        max = Some(0);
                    } else { 
                        return TkEps;           // unexpected case
                    } 
                }
                '}' => return TkBracket (min, max),
                ' ' | '\n' | '\t' => continue, // skip whitespace
                _ => return TkEps,             // unexpected character
            }
        }
        TkEps  // end of input without closing bracket
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
        let mut lexer = Lexer::new("\\a|\\b* (c|d)e?");
        assert_eq!(lexer.next_token(), (Token::TkChar('a')));
        assert_eq!(lexer.next_token(), (Token::TkOr));
        assert_eq!(lexer.next_token(), (Token::TkChar('b')));
        assert_eq!(lexer.next_token(), (Token::TkStar));
        assert_eq!(lexer.next_token(), (Token::TkLparen));
        assert_eq!(lexer.next_token(), (Token::TkChar('c')));
        assert_eq!(lexer.next_token(), (Token::TkOr));
        assert_eq!(lexer.next_token(), (Token::TkChar('d')));
        assert_eq!(lexer.next_token(), (Token::TkRparen));
        assert_eq!(lexer.next_token(), (Token::TkChar('e')));
        assert_eq!(lexer.next_token(), (Token::TkQuestion));
        assert_eq!(lexer.next_token(), (Token::TkEps));
    }

    #[test]
    fn scan_whitespace() {
        let mut lexer = Lexer::new("a\t \n| b+ (c | \td)");
        assert_eq!(lexer.next_token(), (Token::TkChar('a')));
        assert_eq!(lexer.next_token(), (Token::TkOr));
        assert_eq!(lexer.next_token(), (Token::TkChar('b')));
        assert_eq!(lexer.next_token(), (Token::TkPlus));
        assert_eq!(lexer.next_token(), (Token::TkLparen));
        assert_eq!(lexer.next_token(), (Token::TkChar('c')));
        assert_eq!(lexer.next_token(), (Token::TkOr));
        assert_eq!(lexer.next_token(), (Token::TkChar('d')));
        assert_eq!(lexer.next_token(), (Token::TkRparen));
        assert_eq!(lexer.next_token(), (Token::TkEps));
    }

    #[test]
    fn scan_bracket() {
        let mut lexer = Lexer::new("a{2,3}b{0,}c{4}");
        assert_eq!(lexer.next_token(), (Token::TkChar('a')));
        assert_eq!(lexer.next_token(), (Token::TkBracket(2, Some(3))));
        assert_eq!(lexer.next_token(), (Token::TkChar('b')));
        assert_eq!(lexer.next_token(), (Token::TkBracket (0, Some(0))));
        assert_eq!(lexer.next_token(), (Token::TkChar('c')));
        assert_eq!(lexer.next_token(), (Token::TkBracket (4, None)));
        assert_eq!(lexer.next_token(), (Token::TkEps));
    }
}