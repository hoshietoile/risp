use super::ast::*;
use super::lexer::*;
use super::token::*;

pub struct Parser {}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        todo!();
    }

    pub fn parse(&mut self) -> Result<Expr, ExprErr> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let tests = vec![
            "(+ -10 5)",
            "(+ (* 1 2) 3)",
            "(+ (/ 2 (- 10 (* 1 1))))",
            "1",
            "hello",
            "(+ 1 2 (* 1 3))",
            "t",
            "nil",
        ];
        for test in tests {
            let lexer = Lexer::new(String::from(test)).unwrap();
            let mut parser = Parser::new(lexer);
            let expr = parser.parse().unwrap();
            assert_eq!(expr.to_string(), test.to_uppercase());
        }
    }
}
