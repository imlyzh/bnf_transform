

pub type Unit = Vec<Bind>;

#[derive(Debug)]
pub struct Bind(pub String, pub Option<Expr>);

#[derive(Debug)]
pub struct Expr (pub Vec<Alternative>);

#[derive(Debug)]
pub struct Alternative(pub Vec<Term>);

#[derive(Debug)]
pub enum Term {
	Symbol(String),
	Tokens(Token, Option<Token>),
	Group(Expr),
	Option(Expr),
	Repetition(Expr)
}


pub type Token = String;
