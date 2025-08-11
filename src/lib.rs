use std::char;

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
}