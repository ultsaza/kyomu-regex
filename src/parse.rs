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

fn error_msg(expected: &[Token], actual: &Token) -> String {
    let expexted = expected
        .iter()
        .map(|t| format!("'{}'", t))
        .collect::<Vec<_>>()
        .join(", ");
    let actual = match actual {
        Token::TkChar(c) => format!("'{}'", c),
        _ => format!("'{}'", actual),
    };
    format!("Expected one of [{:?}], found {}", expected, actual)
}

impl Parser<'_> {
    fn match_next(&mut self, token: Token) -> Result<()> {
        if self.look == token {
            self.look = self.lexer.next_token();
            Ok(())
        } else {
            Err(error_msg(&[token], &self.look))
        }
    }

    fn factor(&mut self) -> Result<Node> {
        match &self.look {
            Token::TkLparen => {
                self.match_next(Token::TkLparen)?;
                let node = self.sub_expr();
                self.match_next(Token::TkRparen)?;
                node
            }
            Token::TkChar(c) => {
                let node = Node::NdChar(*c);
                self.match_next(Token::TkChar(*c))?;
                node
            }
            _ => Err(error_msg(&[Token::TkLparen, Token::TkChar('_')], &self.look)),
        }
    }

    fn star(&mut self) -> Result<Node> {
        let factor = self.factor();
        match &self.look {
            Token::TkStar => {
                self.match_next(Token::TkStar)?;
                Ok(Node::NdStar(Box::new(factor?)))
            }
            _ => factor,
        }
    }

    fn sub_seq(&mut self) -> Result<Node> {
        let mut left = self.star()?;
        match &self.look {
            Token::TkLparen | Token::TkChar(_) => {
                Ok(
                    Node::NdConcat(
                        Box::new(star?),
                        Box::new(self.sub_seq()?),
                    ),
                )
            }
            _ => star,
        }
    }

    fn seq(&mut self) -> Result<Node> {
        match &self.look {
            Token::TkLparen | Token::TkChar(_) => self.sub_seq(),
            _ => Ok(Node::NdEmpty),
        }
    }

    fn sub_expr(&mut self) -> Result<Node> {
        let mut seq = self.seq()?;
        match &self.look {
            Token::TkOr => {
                self.match_next(Token::TkOr)?;
                OK(
                    Node::NdOr(
                        Box::new(seq?),
                        Box::new(self.sub_expr()?),
                    ),
                )
            }
            _ => Ok(seq),
        }
    }

    fn expr(&mut self) -> Result<Node> {
        let expr = self.sub_expr();
        self.match_next(Token::TkEps)?;
        expr
    }
}