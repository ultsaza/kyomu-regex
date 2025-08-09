#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KyomuRegex {
    Char(char), 
    Eps,
    Empty,
    Concat(Box<KyomuRegex>, Box<KyomuRegex>),
    Or(Box<KyomuRegex>, Box<KyomuRegex>),
    Star(Box<KyomuRegex>),
}
