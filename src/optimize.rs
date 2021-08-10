use crate::ast::*;

pub trait Optimize {
    fn optimize(&self) -> Self;
}

impl Optimize for Term {
    fn optimize(&self) -> Self {
        match self {
            Term::Group(x) => Term::Group(x.optimize()),
            Term::Option(x) => Term::Option(x.optimize()),
            Term::Repetition(x) => Term::Repetition(x.optimize()),
            _ => self.clone(),
        }
    }
}

impl Optimize for Alternative {
    fn optimize(&self) -> Self {
        let r = Alternative(self.0.iter().map(Term::optimize).collect());
        if self.0.len() != 2 {
            return r;
        }
        let _c0 = self.0.get(0).unwrap();
        let c1 = self.0.get(0).unwrap();
        if let Term::Repetition(_x) = c1 {
            todo!("太麻烦了不想写了，跑了")
        } else {
            return r;
        }
    }
}

impl Optimize for Expr {
    fn optimize(&self) -> Self {
        Self(self.0.iter().map(Alternative::optimize).collect())
    }
}

impl Optimize for Bind {
    fn optimize(&self) -> Self {
        let expr = self.1.clone().map(|x| Expr::optimize(&x));
        Self(self.0.clone(), expr)
    }
}

impl Optimize for Unit {
    fn optimize(&self) -> Self {
        self.iter().map(Bind::optimize).collect()
    }
}
