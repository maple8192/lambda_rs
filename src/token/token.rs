#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Lambda,
    Dot,
    Equal,
    Br,
    OpenParen,
    CloseParen,
    Ident(String),
}
