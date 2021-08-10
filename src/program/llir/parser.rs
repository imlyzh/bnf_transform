use pest::iterators::Pair;
use pest::{error::Error, Parser};
use pest_derive::*;

use crate::ast::*;

#[derive(Parser)]
#[grammar = "./program/llir/grammar.pest"]
struct Bnf();

pub trait ParseFrom<T>
where
    Self: std::marker::Sized,
{
    fn parse_from(pair: Pair<T>) -> Self;
}

impl ParseFrom<Rule> for Bind {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::production);
        let mut p = pair.into_inner();
        let name = p.next().unwrap().as_str().to_string();
        let expr = p.next().map(Expr::parse_from);
        Self(name, expr)
    }
}

impl ParseFrom<Rule> for Expr {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::expression);
        let r = pair.into_inner().map(Alternative::parse_from).collect();
        Self(r)
    }
}

impl ParseFrom<Rule> for Alternative {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::alternative);
        let r = pair.into_inner().map(Term::parse_from).collect();
        Self(r)
    }
}

impl ParseFrom<Rule> for Term {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::term);
        let mut ps = pair.into_inner();
        let pair = ps.next().unwrap();
        match pair.as_rule() {
            Rule::production_name => Self::Symbol(pair.as_str().to_string()),
            Rule::group => Self::Group(pair.into_inner().next().map(Expr::parse_from).unwrap()),
            Rule::option => Self::Option(pair.into_inner().next().map(Expr::parse_from).unwrap()),
            Rule::repetition => {
                Self::Repetition(pair.into_inner().next().map(Expr::parse_from).unwrap())
            }
            Rule::token => {
                let a = pair.as_str().to_string();
                let b = ps.next().map(|x| x.as_str().to_string());
                Self::Tokens(a, b)
            }
            _ => unreachable!(),
        }
    }
}

impl ParseFrom<Rule> for Unit {
    fn parse_from(pair: Pair<Rule>) -> Self {
        debug_assert_eq!(pair.as_rule(), Rule::unit);
        pair.into_inner()
            .flat_map(|x| {
                if let Rule::production = x.as_rule() {
                    Some(Bind::parse_from(x))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn parse(i: &str) -> Result<Unit, Error<Rule>> {
    let pair = Bnf::parse(Rule::unit, i)?;
    Ok(pair.flat_map(Unit::parse_from).collect())
}
