use std::{char};
mod parse;
mod lex;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KyomuRegex {
    Char(char),                                 // a single character
    Eps,                                        // ε
    Empty,                                      // ∅
    Concat(Box<KyomuRegex>, Box<KyomuRegex>),   // ⋅ 
    Or(Box<KyomuRegex>, Box<KyomuRegex>),       // +
    Star(Box<KyomuRegex>),                      // *
}

impl KyomuRegex {
    pub fn whole_match(&self, input: &str) -> bool {
        let mut reg = self.clone();
        for ch in input.chars() {
            reg = reg.derivative(ch);
        }
        reg.match_eps()
    }
    pub fn derivative(&self, ch: char) -> KyomuRegex {
        use KyomuRegex::*;
        // Helper to operate or
        fn s_or(left: KyomuRegex, right: KyomuRegex) -> KyomuRegex{
            match (left, right) {
                (l, r) if l == r => l,
                (Empty, r) => r,
                (l, Empty) => l,
                (l, r) => Or(Box::new(l), Box::new(r)),
            }
        }
        // Helper to operate concat
        fn s_concat(left: KyomuRegex, right: KyomuRegex) -> KyomuRegex{
            match (left, right) {
                (Eps, r) => r,
                (l, Eps) => l,
                (Empty, _) | (_, Empty) => Empty,
                (l, r) => Concat(Box::new(l), Box::new(r)),
            }
        }
        match self {
            Char(c) => if *c == ch { Eps } else { Empty },  // D(c) = ε
            Eps => Empty,                                   // D(ε) = ∅      
            Empty => Empty,                                 // D(∅) = ∅
            Concat(left, right) => {
                // D(left ⋅ right) = D(left) ⋅ right + δ(left) ⋅ D(right)
                s_or(
                    s_concat(left.derivative(ch), *right.clone()),
                    s_concat(left.delta(), right.derivative(ch))
                )
            }
            Or(left, right) => {
                // D(left + right) = D(left) + D(right)
                s_or(
                    left.derivative(ch),
                    right.derivative(ch)
                )
            }
            Star(left) => {
                // D(left*) = D(left) ⋅ left*
                s_concat(
                    left.derivative(ch), 
                    Star(left.clone())
                )
            }
        }
    }
    pub fn match_eps(&self) -> bool {
            use KyomuRegex::*;
            match self {
                Char(_) => false,
                Eps => true,
                Empty => false,
                Concat(left, right) => left.match_eps() && right.match_eps(),
                Or(left, right) => left.match_eps() || right.match_eps(),
                Star(_) => true,
            }
        }
    // implementation of δ
    pub fn delta(&self) -> KyomuRegex {
        use KyomuRegex::*;
        if self.match_eps() { Eps } else { Empty }
    }

    fn from_ast(node: crate::parse::Node) -> Self {
        use crate::parse::Node::*;
        use KyomuRegex::*;
        match node {
            NdChar(c) => Char(c),
            NdEps => Eps,
            NdStar(left) => Star(Box::new(Self::from_ast(*left))),
            NdConcat(left, right) => Concat(
                Box::new(Self::from_ast(*left)),
                Box::new(Self::from_ast(*right))
            ),
            NdOr(left, right) => Or(
                Box::new(Self::from_ast(*left)),
                Box::new(Self::from_ast(*right))
            ),
        }
    }

    pub fn compile(pattern: &str) -> Result<Self, String> {
        let mut parser = crate::parse::Parser::new(crate::lex::Lexer::new(pattern));
        let ast = parser.parse()?;
        Ok(Self::from_ast(ast))
    }
}

impl std::str::FromStr for KyomuRegex {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::compile(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    macro_rules! chr { ($ch:expr) => { KyomuRegex::Char($ch) }; }
    macro_rules! concat { ($left:expr, $right:expr) => { KyomuRegex::Concat(Box::new($left), Box::new($right)) }; }
    macro_rules! or { ($left:expr, $right:expr) => { KyomuRegex::Or(Box::new($left), Box::new($right)) }; }
    macro_rules! star { ($left:expr) => { KyomuRegex::Star(Box::new($left)) }; }

    #[test]
    fn simple_match() {
        // (a+b)*(ab)
        let r = concat!(
                    star!(
                        or!(
                            chr!('a'),
                            chr!('b')
                        )
                    ),
                    concat!(
                        chr!('a'),
                        chr!('b')
                    )
                );
        assert!( r.whole_match("ab") );
        assert!( r.whole_match("aab") );
        assert!( r.whole_match("babab") );
        assert!(!r.whole_match("aba") );
        assert!(!r.whole_match("abc") );
    }

    #[test]
    fn parse_and_match_from_string() {
        let r:KyomuRegex = "a|(bc)*".parse().unwrap();
        assert!(r.whole_match(""));
        assert!(r.whole_match("a"));
        assert!(r.whole_match("bc"));
        assert!(r.whole_match("bcbc"));
        assert!(!r.whole_match("b"));
        assert!(!r.whole_match("abc"));
    }
}