use crate::modules::ast::{Expr, Lambda};
use crate::modules::error::RispError;
use std::{collections::HashMap, rc::Rc};

pub type ExprEnv = HashMap<String, Expr>;

pub struct Evaluator {}

macro_rules! basic_op {
    ($fn: expr) => {
        |args: &[Expr]| -> Result<Expr, RispError> {
            let floats = parse_list_of_floats(args)?;
            let (first, rest) = floats
                .split_first()
                .ok_or_else(|| RispError::Expr("Expected at least one number".to_string()))?;
            Ok(Expr::Number(rest.iter().fold(*first, $fn)))
        }
    };
}

fn parse_list_of_floats(args: &[Expr]) -> Result<Vec<f64>, RispError> {
    args.iter()
        .map(|x| match x {
            Expr::Number(num) => Ok(*num),
            _ => Err(RispError::Expr(format!("{} is not number", x))),
        })
        .collect()
}

fn parse_list_of_symbols(args: &[Expr]) -> Result<Vec<String>, RispError> {
    args.iter()
        .map(|x| match x {
            Expr::Symbol(symbol) => Ok(symbol.clone()),
            _ => Err(RispError::Expr(format!("{} is not symbol", x))),
        })
        .collect()
}

pub fn default_env() -> ExprEnv {
    let mut env: ExprEnv = HashMap::new();
    env.insert("+".to_string(), Expr::Func(basic_op!(|sum, x| sum + x)));
    env.insert("-".to_string(), Expr::Func(basic_op!(|sum, x| sum - x)));
    env.insert("*".to_string(), Expr::Func(basic_op!(|sum, x| sum * x)));
    env.insert("/".to_string(), Expr::Func(basic_op!(|sum, x| sum / x)));
    env
}

impl Evaluator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval(&mut self, expr: &Expr, env: &mut ExprEnv) -> Result<Expr, RispError> {
        match expr {
            Expr::String(_) => Ok(expr.clone()),
            Expr::Number(_) => Ok(expr.clone()),
            Expr::Nil => Ok(expr.clone()),
            Expr::True => Ok(expr.clone()),
            Expr::Symbol(sym) => match env.get(sym) {
                Some(expr) => Ok(expr.clone()),
                None => Err(RispError::Expr(format!(
                    "not found symbol: {}, env: {}",
                    sym,
                    self.print_env(env.clone())
                ))),
            },
            Expr::List(list) => {
                let (first, rest) = list
                    .split_first()
                    .ok_or_else(|| RispError::Expr("Expected at least one number".to_string()))?;
                match self.eval_builtin(first, rest, env) {
                    Some(expr) => expr,
                    None => {
                        let expr = self.eval(first, env)?;
                        match expr {
                            Expr::Func(f) => f(self.eval_args(rest, env)?.as_slice()),
                            Expr::Lambda(lambda) => self.eval_lambda(lambda, rest, env),
                            _ => unreachable!(),
                        }
                    }
                }
            }
            _ => Err(RispError::Expr(format!("Invalid expr: {}", expr))),
        }
    }

    pub fn eval_args(&mut self, args: &[Expr], env: &mut ExprEnv) -> Result<Vec<Expr>, RispError> {
        args.iter().map(|x| self.eval(x, env)).collect()
    }

    pub fn eval_builtin(
        &mut self,
        first: &Expr,
        args: &[Expr],
        env: &mut ExprEnv,
    ) -> Option<Result<Expr, RispError>> {
        match first {
            Expr::Symbol(symbol) => match symbol.as_str() {
                "SETQ" => Some(self.eval_setq(args, env)),
                "DEFUN" => Some(self.eval_defun(args, env)),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn eval_lambda(
        &mut self,
        lambda: Lambda,
        outer_args: &[Expr],
        env: &mut ExprEnv,
    ) -> Result<Expr, RispError> {
        if lambda.args.len() != outer_args.len() {
            return Err(RispError::Expr(
                "Number of args and lambda's arg is not same".to_string(),
            ));
        }

        let mut args = Vec::<Expr>::new();
        for arg in outer_args {
            args.push(self.eval(arg, env)?);
        }

        let mut local_env = env.clone();
        for (i, k) in lambda.args.iter().enumerate() {
            let value = args
                .get(i)
                .ok_or(RispError::Expr("Not found value from env".to_string()))?;
            local_env.insert(k.clone(), value.clone());
        }

        let result = self.eval(&lambda.body, &mut local_env)?;
        Ok(result)
    }

    pub fn eval_defun(&mut self, args: &[Expr], env: &mut ExprEnv) -> Result<Expr, RispError> {
        if args.len() != 3 {
            return Err(RispError::Expr(
                "Unexpected function definition".to_string(),
            ));
        }
        let mut iter = args.iter();
        let symbol = iter
            .next()
            .ok_or(RispError::Expr("Cannot get function name".to_string()))?;
        let name = match symbol {
            Expr::Symbol(symbol) => Ok(symbol),
            _ => Err(RispError::Expr(format!("Invalid symbol: {}", symbol))),
        }?;
        let args_expr = iter
            .next()
            .ok_or(RispError::Expr("Cannot get function args".to_string()))?;
        let args = match args_expr {
            Expr::List(list) => Ok(list),
            _ => Err(RispError::Expr(format!("Invalid list: {}", args_expr))),
        }?;

        let args = parse_list_of_symbols(args)?;
        let body = iter
            .next()
            .ok_or(RispError::Expr("Cannot get function body".to_string()))?;
        let lambda = Expr::Lambda(Lambda {
            args,
            body: Rc::new(body.clone()),
        });
        env.insert(name.clone(), lambda);
        Ok(Expr::String(name.clone()))
    }

    pub fn eval_setq(&mut self, args: &[Expr], env: &mut ExprEnv) -> Result<Expr, RispError> {
        let first = args
            .first()
            .ok_or(RispError::Expr("Expected first arg".to_string()))?;
        let key = match first {
            Expr::Symbol(sym) => Ok(sym.clone()),
            _ => Err(RispError::Expr("First arg must be symbol".to_string())),
        }?;
        let second = args
            .get(1)
            .ok_or(RispError::Expr("Expected second arg".to_string()))?;
        let value = self.eval(second, env)?;

        env.insert(key, value.clone());
        Ok(value.clone())
    }

    fn print_env(&mut self, env: ExprEnv) -> String {
        env.clone()
            .iter()
            .map(|x| format!("{}={}", x.0, x.1))
            .collect::<Vec<String>>()
            .join(" ")
    }
}
