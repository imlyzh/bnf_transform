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
            Term::Option(expr) => {
                let r = expr.gen();
                if r.contains(" ") {
                    if  r.chars().nth(0).unwrap() == '(' &&
                        r.chars().nth(r.len()-1).unwrap() == ')' {
                        format!("{}?", r)
                    } else {
                        format!("({})?", r)
                    }
                } else {
                    format!("{}?", r)
                }
            },
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
            Term::Group(expr) => {
                let r = expr.gen();
                if r.contains(" ") {
                    if  r.chars().nth(0).unwrap() == '(' &&
                        r.chars().nth(r.len()-1).unwrap() == ')' {
                        r
                    } else {
                        format!("({})", r)
                    }
                } else {
                    r
                }
            },
            Term::Repetition(expr) => {
                let r = expr.gen();
                if r.contains(" ") {
                    if  r.chars().nth(0).unwrap() == '(' &&
                        r.chars().nth(r.len()-1).unwrap() == ')' {
                        format!("{}*", r)
                    } else {
                        format!("({})*", r)
                    }
                } else {
                    format!("{}*", r)
                }
            },
            Term::OneOrMore(expr) => {
                let r = expr.gen();
                if r.contains(" ") {
                    if  r.chars().nth(0).unwrap() == '(' &&
                        r.chars().nth(r.len()-1).unwrap() == ')' {
                            format!("{}+", r)
                    } else {
                        format!("({})+", r)
                    }
                } else {
                    format!("{}+", r)
                }
            },
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
        if self.0 == "!comment" {
            format!("COMMENT = _{{ {} }}", self.1.clone().unwrap().gen())
        } else if self.0 == "!whitespace" {
            format!("WHITESPACE = _{{ {} }}", self.1.clone().unwrap().gen())
        } else if self.0.chars().nth(0).unwrap() == '_' {
            format!("{} = @{{ {} }}", self.0, self.1.clone().unwrap().gen())
        } else {
            format!("{} = {{ {} }}", self.0, self.1.clone().unwrap().gen())
        }
    }
}

impl Gen for Unit {
    fn gen(&self) -> String {
        let r: Vec<String> = self.iter().map(Bind::gen).collect();
        r.join("\n")
    }
}
