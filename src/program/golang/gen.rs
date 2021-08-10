use crate::ast::*;

pub trait Gen {
    fn gen(&self) -> String;
}

impl Gen for Term {
    fn gen(&self) -> String {
        match self {
            Term::Symbol(x) => x.to_lowercase(),
            Term::Group(expr) => format!("({})", expr.gen()),
            Term::Option(expr) => format!("({})?", expr.gen()),
            Term::Repetition(expr) => format!("({})*", expr.gen()),
            Term::Tokens(left, right) => {
                if let Some(right) = right {
                    format!("{}..{}", left, right)
                    /*
                    format!(
                        "'{}'..'{}'",
                        left.chars().nth(0).unwrap(),
                        right.chars().nth(0).unwrap()
                    )
                    // */
                } else {
                    if left.chars().nth(0).unwrap() == '`' {
                        format!("\"\\{}\"", left.chars().nth(1).unwrap())
                    } else {
                        format!("{}", left)
                    }
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
        let name = self.0.to_lowercase();
        let expr = if let Some(expr) = &self.1 {
            expr.gen()
        } else {
            "UNDEFINED".to_string()
        };
        format!("{} = {{ {} }}", name, expr)
    }
}

impl Gen for Unit {
    fn gen(&self) -> String {
        let r: Vec<String> = self.iter().map(Bind::gen).collect();
        r.join("\n")
    }
}
