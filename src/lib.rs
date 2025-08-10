use std::char;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KyomuRegex {
    Char(char), 
    Eps,
    Empty,
    Concat(Box<KyomuRegex>, Box<KyomuRegex>),
    Or(Box<KyomuRegex>, Box<KyomuRegex>),
    Star(Box<KyomuRegex>),
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
        fn s_or(left: KyomuRegex, right: KyomuRegex) -> KyomuRegex{
            if left == right {
                left
            } else if left == Empty {
                right 
            } else if right == Empty {
                left
            } else {
                Or(Box::new(left), Box::new(right))
            }
        }
        fn s_concat(left: KyomuRegex, right: KyomuRegex) -> KyomuRegex{
            if left == Eps {
                right
            } else if right == Eps{
                left
            } else if  left == Empty || right == Empty {
                Empty
            } else {
                Concat(Box::new(left), Box::new(right))
            }
        }
        match self {
            Char(c) => {
                if *c == ch {
                    Eps
                } else {
                    Empty
                }
            }

            Eps => Empty,

            Empty => Empty,

            Concat(left, right) => {
                s_or(
                    s_concat(left.derivative(ch), *right.clone()),
                    s_concat(left.delta(), right.derivative(ch))
                )
            }
            Or(left, right) => {
                s_or(
                    left.derivative(ch),
                    right.derivative(ch)
                )
            }
            Star(left) => {
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
    pub fn delta(&self) -> KyomuRegex {
        if self.match_eps() {
            KyomuRegex::Eps
        } else {
            KyomuRegex::Empty
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! chr { ($ch:expr) => { KyomuRegex::Char($ch) }; }
    macro_rules! eps { () => { KyomuRegex::Eps }; }
    macro_rules! empty { () => { KyomuRegex::Empty }; }
    macro_rules! concat { ($left:expr, $right:expr) => { KyomuRegex::Concat(Box::new($left), Box::new($right)) }; }
    macro_rules! or { ($left:expr, $right:expr) => { KyomuRegex::Or(Box::new($left), Box::new($right)) }; }
    macro_rules! star { ($left:expr) => { KyomuRegex::Star(Box::new($left)) }; }

    #[test]
    fn simple_match() {
        let r = concat!( star!( or!( chr!('a'), chr!('b') ) ),
                      concat!( chr!('a'), chr!('b') ) );
        assert!( r.whole_match("ab") );
        assert!( r.whole_match("aab") );
        assert!( r.whole_match("babab") );
        assert!(!r.whole_match("aba") );
    }

}