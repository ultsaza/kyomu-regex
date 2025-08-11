use crate::lex::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    NdChar(char),
    NdEmpty,
    NdStar(Box<Node>),
    NdOr(Box<Node>, Box<Node>),
    NdConcat(Box<Node>, Box<Node>),
}
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    look: Token,
}
type Result<T> = std::result::Result<T, String>;

fn error_msg(expexted: &[Token], actual: &Token) -> String {
    let expexted = exptected
        .iter()
        .map(|t| format!("'{}'", t))
        .collect::<Vec<_>>()
        .join(", ");
    let actual = match actual {
        Token::TkChar(c) => format!("'{}'", c),
        _ => format!("'{}'", actual),
    };
    format!("Expected one of [{}], found {}", expected, actual)
}
