use std::char;
mod lex;
mod parse;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KyomuRegex {
    Char(char),                               // a single character
    Eps,                                      // ε
    Empty,                                    // ∅
    Concat(Box<KyomuRegex>, Box<KyomuRegex>), // ⋅
    Or(Box<KyomuRegex>, Box<KyomuRegex>),     // |
    Star(Box<KyomuRegex>),                    // *
    Plus(Box<KyomuRegex>),                    // +
    Question(Box<KyomuRegex>),                // ?
    Bracket(u32, Option<u32>, Box<KyomuRegex>),       // {min, max}
}

impl KyomuRegex {
    pub fn whole_match(&self, input: &str) -> bool {
        let mut reg = self.clone();
        for ch in input.chars() {
            reg = reg.derivative(ch);
        }
        reg.match_eps()
    }
    pub fn derivative(&self, ch: char) -> Self {
        use KyomuRegex::*;
        // Helper to operate or
        fn s_or(left: KyomuRegex, right: KyomuRegex) -> KyomuRegex {
            match (left, right) {
                (l, r) if l == r => l,
                (Empty, r) => r,
                (l, Empty) => l,
                (l, r) => Or(Box::new(l), Box::new(r)),
            }
        }
        // Helper to operate concat
        fn s_concat(left: KyomuRegex, right: KyomuRegex) -> KyomuRegex {
            match (left, right) {
                (Eps, r) => r,
                (l, Eps) => l,
                (Empty, _) | (_, Empty) => Empty,
                (l, r) => Concat(Box::new(l), Box::new(r)),
            }
        }
        match self {
            Char(c) => {
                if *c == '.' || *c == ch {
                    Eps
                } else {
                    Empty
                }
            } // D(c) = ε
            Eps => Empty,   // D(ε) = ∅
            Empty => Empty, // D(∅) = ∅
            Concat(left, right) => {
                // D(left ⋅ right) = D(left) ⋅ right | δ(left) ⋅ D(right)
                s_or(
                    s_concat(left.derivative(ch), *right.clone()),
                    s_concat(left.delta(), right.derivative(ch)),
                )
            }
            Or(left, right) => {
                // D(left | right) = D(left) | D(right)
                s_or(left.derivative(ch), right.derivative(ch))
            }
            Star(left) => {
                // D(left*) = D(left) ⋅ left*
                s_concat(left.derivative(ch), Star(left.clone()))
            }
            Plus(left) => {
                // D(left+) = D(left) ⋅ left* | δ(left) ⋅ D(left) ⋅ left*
                s_concat(
                    s_or(
                        left.derivative(ch),
                        s_concat(left.delta(), left.derivative(ch)),
                    ),
                    Star(left.clone()),
                )
            }
            Question(left) => {
                // D(left?) = D(left)
                left.derivative(ch)
            }
            Bracket(min, max, r) => {
                // D(r{min, max}) = D(r) ⋅ r{min-1, max-1} | δ(r) ⋅ r{min-1, max-1}
                match (min, max) {
                    // D(ε) = ∅ 
                    (0, Some(0)) => Empty,
                    // D(r{min, infty}) = D(r{min} ⋅ r* )
                    (_, Some(0)) => {
                        let res = (0..*min).fold(Eps, |acc, _| s_concat(*r.clone(), acc));
                        s_concat(res, Star(r.clone())).derivative(ch)
                    }
                    // D(r{min})
                    (min, None) => {
                        let res = (0..*min).fold(Eps, |acc, _| s_concat(*r.clone(), acc));
                        res.derivative(ch)
                    }

                    // invalid case (e.g., {4,2})
                    (_, _) if *min > max.unwrap() => Empty,

                    // D(r{min, max}) = D(r) ⋅ r{min-1, max-1} | δ(r) ⋅ r{min-1, max-1}
                    (_, _) => s_or(
                        s_concat(
                            r.derivative(ch),
                            Bracket(min.saturating_sub(1), Some(max.unwrap().saturating_sub(1)), r.clone()),
                        ),
                        s_concat(
                            r.delta(),
                            Bracket(min.saturating_sub(1), Some(max.unwrap().saturating_sub(1)), r.clone()),
                        ),
                    ),
                }
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
            Plus(r) => r.match_eps(),
            Question(_) => true,
            Bracket(min, _, r) => *min == 0 || r.match_eps(),
        }
    }
    // implementation of δ
    pub fn delta(&self) -> KyomuRegex {
        use KyomuRegex::*;
        if self.match_eps() {
            Eps
        } else {
            Empty
        }
    }

    fn build_from_ast(node: crate::parse::Node) -> Self {
        use crate::parse::Node::*;
        use KyomuRegex::*;
        match node {
            NdChar(c) => Char(c),
            NdEps => Eps,
            NdStar(left) => Star(Box::new(Self::build_from_ast(*left))),
            NdPlus(left) => Plus(Box::new(Self::build_from_ast(*left))),
            NdQuestion(left) => Question(Box::new(Self::build_from_ast(*left))),
            NdConcat(left, right) => Concat(
                Box::new(Self::build_from_ast(*left)),
                Box::new(Self::build_from_ast(*right)),
            ),
            NdOr(left, right) => Or(
                Box::new(Self::build_from_ast(*left)),
                Box::new(Self::build_from_ast(*right)),
            ),
            NdBracket(min, max, r) => Bracket(min, max, Box::new(Self::build_from_ast(*r))),
        }
    }

    pub fn compile(pattern: &str) -> Result<Self, String> {
        let mut parser = crate::parse::Parser::new(crate::lex::Lexer::new(pattern));
        let ast = parser.parse()?;
        Ok(Self::build_from_ast(ast))
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

    macro_rules! chr {
        ($ch:expr) => {
            KyomuRegex::Char($ch)
        };
    }
    macro_rules! concat {
        ($left:expr, $right:expr) => {
            KyomuRegex::Concat(Box::new($left), Box::new($right))
        };
    }
    macro_rules! or {
        ($left:expr, $right:expr) => {
            KyomuRegex::Or(Box::new($left), Box::new($right))
        };
    }
    macro_rules! star {
        ($left:expr) => {
            KyomuRegex::Star(Box::new($left))
        };
    }

    #[test]
    fn simple_match() {
        // (a+b)*(ab)
        let r = concat!(
            star!(or!(chr!('a'), chr!('b'))),
            concat!(chr!('a'), chr!('b'))
        );
        assert!(r.whole_match("ab"));
        assert!(r.whole_match("aab"));
        assert!(r.whole_match("babab"));
        assert!(!r.whole_match("aba"));
        assert!(!r.whole_match("abc"));
    }

    #[test]
    fn parse_and_match_from_string() {
        let r: KyomuRegex = "a|(bc)*".parse().unwrap();
        assert!(r.whole_match(""));
        assert!(r.whole_match("a"));
        assert!(r.whole_match("bc"));
        assert!(r.whole_match("bcbc"));
        assert!(!r.whole_match("b"));
        assert!(!r.whole_match("abc"));
    }

    #[test]
    fn parse_wild_card() {
        let r: KyomuRegex = "a.*b".parse().unwrap();
        assert!(r.whole_match("ab"));
        assert!(r.whole_match("abcdb"));
        assert!(!r.whole_match("a123c"));
        assert!(!r.whole_match("b"));
    }

    #[test]
    fn parse_plus() {
        let r: KyomuRegex = "(ab)+c".parse().unwrap();
        assert!(r.whole_match("abc"));
        assert!(r.whole_match("ababababababababc"));
        assert!(!r.whole_match("c"));
        assert!(!r.whole_match("ac"));
    }

    #[test]
    fn parse_question() {
        let r: KyomuRegex = "a?b".parse().unwrap();
        assert!(r.whole_match("b"));
        assert!(r.whole_match("ab"));
        assert!(!r.whole_match("a"));
        assert!(!r.whole_match("aa"));
    }

    #[test]
    fn parse_bracket() {
        let r: KyomuRegex = "a{2,3}b".parse().unwrap();
        assert!(r.whole_match("aab"));
        assert!(r.whole_match("aaab"));
        assert!(!r.whole_match("a"));
        assert!(!r.whole_match("aa"));
        assert!(!r.whole_match("aaa"));
        assert!(!r.whole_match("b"));
        let r: KyomuRegex = "a{0}b".parse().unwrap(); // == a?b
        assert!(r.whole_match("b"));
        assert!(!r.whole_match("ab"));
        assert!(!r.whole_match("a"));
        assert!(!r.whole_match("aa"));
        let r: KyomuRegex = "a{2,}b".parse().unwrap();
        assert!(r.whole_match("aab"));
        assert!(r.whole_match("aaab"));
        assert!(r.whole_match("aaaab"));
        assert!(!r.whole_match("a"));
        assert!(!r.whole_match("b"));
        let r: KyomuRegex = "a{2}".parse().unwrap();
        assert!(r.whole_match("aa"));
        assert!(!r.whole_match("a"));
        assert!(!r.whole_match("aaa"));
    }
}
