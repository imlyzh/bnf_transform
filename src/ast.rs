

pub type Unit = Vec<Bind>;

#[derive(Debug, Clone)]
pub struct Bind(pub String, pub Option<Expr>);

#[derive(Debug, Clone)]
pub struct Expr (pub Vec<Alternative>);

#[derive(Debug, Clone)]
pub struct Alternative(pub Vec<Term>);

#[derive(Debug, Clone)]
pub enum Term {
	Symbol(String),
	Tokens(Token, Option<Token>),
	Group(Expr),
	Option(Expr),
	Repetition(Expr)
}


pub type Token = String;
