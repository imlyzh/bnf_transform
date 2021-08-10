pub type Unit = Vec<Bind>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bind(pub String, pub Option<Expr>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr(pub Vec<Alternative>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alternative(pub Vec<Term>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Symbol(String),
    Tokens(Token, Option<Token>),
    Group(Expr),
    Option(Expr),
    Repetition(Expr),
}

pub type Token = String;
