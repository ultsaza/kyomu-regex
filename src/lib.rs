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
        return reg.match_eps();
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

            _ => {
                unreachable!()
            }
        }
    }
}

