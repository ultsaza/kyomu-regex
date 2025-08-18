use crate::lex::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    NdChar(char),
    NdEps,
    NdStar(Box<Node>),
    NdPlus(Box<Node>),
    NdOr(Box<Node>, Box<Node>),
    NdQuestion(Box<Node>),
    NdConcat(Box<Node>, Box<Node>),
    NdBracket (u32, u32, Box<Node>),
}
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    look: Token,
}
type Result<T> = std::result::Result<T, String>;

fn error_msg(expected: &[Token], actual: &Token) -> String {
    let expected = expected
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
    pub fn new(mut lexer: Lexer) -> Parser{
        let node = lexer.next_token();
        Parser {
            lexer,
            look: node,
        }
    }

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
                Ok(node)
            }
            _ => Err(error_msg(&[Token::TkLparen, Token::TkChar('_')], &self.look)),
        }
    }

    fn quantifier(&mut self) -> Result<Node> {
        let factor = self.factor();
        match &self.look {
            Token::TkStar => {
                self.match_next(Token::TkStar)?;
                Ok(Node::NdStar(Box::new(factor?)))
            }
            Token::TkPlus => {
                self.match_next(Token::TkPlus)?;
                Ok(Node::NdPlus(Box::new(factor?)))
            }
            Token::TkQuestion => {
                self.match_next(Token::TkQuestion)?;
                Ok(Node::NdQuestion(Box::new(factor?)))
            }
            _ => factor,
        }
    }

    fn sub_seq(&mut self) -> Result<Node> {
        let quantifier = self.quantifier();
        match &self.look {
            Token::TkLparen | Token::TkChar(_) => {
                Ok(
                    Node::NdConcat(
                        Box::new(quantifier?),
                        Box::new(self.sub_seq()?),
                    ),
                )
            }
            _ => quantifier,
        }
    }

    fn seq(&mut self) -> Result<Node> {
        match &self.look {
            Token::TkLparen | Token::TkChar(_) => self.sub_seq(),
            _ => Ok(Node::NdEps),
        }
    }

    fn sub_expr(&mut self) -> Result<Node> {
        let seq = self.seq()?;
        match &self.look {
            Token::TkOr => {
                self.match_next(Token::TkOr)?;
                Ok(
                    Node::NdOr(
                        Box::new(seq),
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

    pub fn parse(&mut self) -> Result<Node> {
        self.expr()
    }
}

#[cfg(test)]
mod tests {
    use crate::lex::*;
    use crate::parse::*;

    #[test]
    fn expression() {
        let mut parser = Parser::new(Lexer::new(r"a|(bc)*"));
        assert_eq!(
            parser.expr(),
            Ok(Node::NdOr(
                Box::new(Node::NdChar('a')),
                Box::new(Node::NdStar(Box::new(Node::NdConcat(
                    Box::new(Node::NdChar('b')),
                    Box::new(Node::NdChar('c'))
                ))))
            ))
        );
    }

    #[test]
    fn plus_operator() {
        let mut parse = Parser::new(Lexer::new(r"a+b?"));
        assert_eq!(
            parse.expr(),
            Ok(Node::NdConcat(
                Box::new(Node::NdPlus(Box::new(Node::NdChar('a')))),
                Box::new(Node::NdQuestion(Box::new(Node::NdChar('b'))))
            ))
        );
    }
}
