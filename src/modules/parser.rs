use super::ast::*;
use super::error::*;
use super::lexer::*;
use super::token::*;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<Expr, RispError> {
        let token = self.lexer.next_token()?;
        match token {
            Token::NUMBER(num) => Ok(Expr::Number(num)),
            Token::STRING(s) => Ok(Expr::String(s)),
            Token::LITERAL(symbol) => Ok(Expr::Symbol(symbol)),
            Token::ASTERISK => Ok(Expr::Symbol("*".to_string())),
            Token::MINUS => Ok(Expr::Symbol("-".to_string())),
            Token::PLUS => Ok(Expr::Symbol("+".to_string())),
            Token::SLASH => Ok(Expr::Symbol("/".to_string())),
            Token::TRUE => Ok(Expr::True),
            Token::NIL => Ok(Expr::Nil),
            Token::ILLEGAL(token) => Err(RispError::Expr(format!("Invalid token: {}", token))),
            Token::EOF | Token::RPAREN => Ok(Expr::Nil),
            Token::LPAREN => {
                let mut list = Vec::<Expr>::new();
                loop {
                    match self.parse() {
                        Ok(expr) => {
                            if expr == Expr::Nil {
                                return Ok(Expr::List(list));
                            }
                            list.push(expr);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }
        }
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
