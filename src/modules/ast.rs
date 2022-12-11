use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum ExprErr {
    Cause(String),
}

#[derive(Clone)]
pub struct Lambda {
    pub args: Vec<String>,
    pub body: Rc<Expr>,
}

#[derive(Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Expr>),
    True,
    Nil,
    Func(fn(&[Expr]) -> Result<Expr, ExprErr>),
    Lambda(Lambda),
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        use Expr::{List, Nil, Number, String, Symbol};
        match (self, other) {
            (Symbol(a), Symbol(b)) => a == b,
            (Number(a), Number(b)) => a == b,
            (String(a), String(b)) => a == b,
            (List(a), List(b)) => a == b,
            (Nil, Nil) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Expr::List(exprs) => {
                let xs = exprs.iter().map(|x| x.to_string()).collect::<Vec<String>>();
                format!("({})", xs.join(" "))
            }
            Expr::Number(num) => num.to_string(),
            Expr::String(s) => s.to_string(),
            Expr::Symbol(sym) => sym.to_string(),
            Expr::Nil => "NIL".to_string(),
            Expr::Func(_) => "LAMBDA".to_string(),
            Expr::Lambda(_) => "LAMBDA".to_string(),
            Expr::True => "T".to_string(),
        };
        write!(f, "{}", s)
    }
}
