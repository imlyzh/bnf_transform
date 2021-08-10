use crate::ast::*;

pub trait LeftRecOptimize {
    fn left_rec_optimize(self) -> Self;
}

impl LeftRecOptimize for Bind {
    fn left_rec_optimize(self) -> Self {
        let Bind(name, expr) = self;
        if let Some(expr) = &expr {
            if let Some(r) = is_left_rec(&name, expr) {
                return Self(name, Some(Expr(vec![r])));
            }
        }
        Self(name, expr)
    }
}

fn is_left_rec(name: &String, expr: &Expr) -> Option<Alternative> {
    // Type | TypeList ~ "," ~ Type
    if expr.0.len() != 2 {
        return None;
    }
    // Type
    let first = expr.0.first().unwrap();
    if first.0.len() != 1 {
        return None;
    }
    // Type
    let first = first.0.first().unwrap();
    // TypeList ~ "," ~ Type
    let last_line = expr.0.last().unwrap();
    if last_line.0.len() <= 1 {
        return None;
    }
    // TypeList
    if let Term::Symbol(sym) = last_line.0.first().unwrap() {
        if sym != name {
            return None;
        }
    } else {
        return None;
    }
    // "," ~ Type
    let r = &last_line.0[1..].to_vec();
    // ("," ~ Type)*
    let r = Term::Repetition(Expr(vec![Alternative(r.clone())]));
    // Type ~ ("," ~ Type)*
    Some(Alternative(vec![first.clone(), r]))
}

impl LeftRecOptimize for Unit {
    fn left_rec_optimize(self) -> Self {
        self.into_iter().map(Bind::left_rec_optimize).collect()
    }
}


