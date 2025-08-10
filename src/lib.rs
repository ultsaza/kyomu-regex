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
                let left = Concat(
                    Box::new(left.derivative(ch)), 
                    right.clone()
                );
                if left.match_eps() {
                    let right = right.derivative(ch);
                    Or(
                        Box::new(left), 
                        Box::new(right)
                    )
                } else {
                    left
                }
            }
            Or(left, right) => {
                Or(
                    Box::new(left.derivative(ch)),
                    Box::new(right.derivative(ch)),
                )
            }
            Star(left) => {
                Concat(
                    Box::new(left.derivative(ch)),
                    Box::new(Star(left.clone())),
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

