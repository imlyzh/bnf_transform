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

pub trait OneOrMoreOptimize {
    fn one_or_more_optimize(self) -> Self;
}

impl OneOrMoreOptimize for Unit {
    fn one_or_more_optimize(self) -> Self {
        self.into_iter().map(Bind::one_or_more_optimize).collect()
    }
}

impl OneOrMoreOptimize for Bind {
    fn one_or_more_optimize(self) -> Self {
        Self(self.0, self.1.map(Expr::one_or_more_optimize))
    }
}

impl OneOrMoreOptimize for Expr {
    fn one_or_more_optimize(self) -> Self {
        let r = self.0.into_iter().map(Alternative::one_or_more_optimize).collect();
        Self(r)
    }
}

impl OneOrMoreOptimize for Alternative {
    fn one_or_more_optimize(self) -> Self {
        let r: Vec<Term> = self.0.into_iter().map(Term::one_or_more_optimize).collect();
        if r.len() != 2 {
            return Self(r);
        }
        let first = r.first().unwrap();
        if let Term::Repetition(last) = r.last().unwrap() {
            if last.0.len() != 1 {
                return Self(r);
            }
            let unboxed_last = last.0.first().unwrap();
            if last.0.len() != 1 {
                return Self(r);
            }
            let unboxed_last = unboxed_last.0.first().unwrap();
            if first == unboxed_last {
                // let expr = Expr(vec![Alternative(vec![first.clone()])]);
                return Self(vec![Term::OneOrMore(last.clone())]);
            }
        }
        Self(r)
    }
}

impl OneOrMoreOptimize for Term {
    fn one_or_more_optimize(self) -> Self {
        match self {
            Term::Group(expr) => Term::Group(expr.one_or_more_optimize()),
            Term::Option(expr) => Term::Option(expr.one_or_more_optimize()),
            Term::Repetition(expr) => Term::Repetition(expr.one_or_more_optimize()),
            Term::OneOrMore(expr) => Term::OneOrMore(expr.one_or_more_optimize()),
            _ => self
        }
    }
}