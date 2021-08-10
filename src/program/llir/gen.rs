use crate::ast::*;

pub trait Gen {
    fn gen(&self) -> String;
}

impl Gen for Term {
    fn gen(&self) -> String {
        match self {
            Term::Symbol(x) => if x == "." {
                "ANY"
            } else {
                x.as_str()
            }.to_string(),
            Term::Group(expr) => format!("({})", expr.gen()),
            Term::Option(expr) => format!("({})?", expr.gen()),
            Term::Repetition(expr) => format!("({})*", expr.gen()),
            Term::Tokens(left, right) => {
                if let Some(right) = right {
                    format!("{}..{}", left, right)
                } else {
                    let r = &left[1..left.len()-1];
                    if ["\"", "\'", "\\", "\t", "\r"].contains(&r) {
                        format!("\"\\{}\"", r)
                    } else {
                        format!("\"{}\"", r)
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
        if r.first().unwrap().as_str() == "empty" {
            format!("({})?", r[1..].join(" | "))
        } else {
            r.join(" | ")
        }
    }
}

impl Gen for Bind {
    fn gen(&self) -> String {
        let name = if self.0 == "!comment" {
            "COMMENT"
        } else if self.0 == "!whitespace" {
            "WHITESPACE"
        } else {
            self.0.as_str()
        }.to_string();
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
