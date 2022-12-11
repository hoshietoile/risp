mod modules;

use modules::lexer;
use modules::parser;

// TODO: need to implement eval with real struct.
fn eval() -> () {}

fn main() {
    println!("Hello, world!");
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
        let lexer = lexer::Lexer::new(String::from(test)).unwrap();
        let mut parser = parser::Parser::new(lexer);
        let expr = parser.parse().unwrap();
        println!("{}", expr.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test() {}

    #[test]
    fn eval_basic_atom() {}

    #[test]
    fn eval_calc() {}

    #[test]
    fn eval_symbol() {}

    #[test]
    fn eval_func() {}
}
