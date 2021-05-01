use std::collections::HashSet;

use crate::ast::*;

pub trait Gen {
    fn gen(&self) -> String;
}

impl Gen for Term {
    fn gen(&self) -> String {
        match self {
            Term::Symbol(x) => x.clone(),
			Term::Group(expr) => format!("({})", expr.gen()),
            Term::Option(expr) => format!("({})?", expr.gen()),
            Term::Repetition(expr) => format!("({})*", expr.gen()),
            Term::Tokens(left, right) => {
				if let Some(right) = right {
					assert_eq!(left.len(), 1);
					assert_eq!(right.len(), 1);
					format!("'{}'..'{}'",
						left.chars().nth(0).unwrap(),
						right.chars().nth(0).unwrap())
				} else {
					format!("{}", left)
				}
			}
        }
    }
}

impl Gen for Alternative {
    fn gen(&self) -> String {
        let r: Vec<String> = self.0.iter().map(Term::gen).collect();
		r.join(" ~ ")
    }
}

impl Gen for Expr {
    fn gen(&self) -> String {
        let r: Vec<String> = self.0.iter().map(Alternative::gen).collect();
		r.join(" | ")
    }
}

impl Gen for Bind {
    fn gen(&self) -> String {
        let expr = if let Some(expr) = &self.1 {
            expr.gen()
        } else {
            "UNDEFINED".to_string()
        };
        format!("{} = {{ {} }}", self.0, expr)
    }
}

impl Gen for Unit {
    fn gen(&self) -> String {
        let r: Vec<String> = self.iter().map(Bind::gen).collect();
		r.join("\n")
    }
}
