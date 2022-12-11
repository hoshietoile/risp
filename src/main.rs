mod modules;

use std::fs::File;
use std::io::BufRead;
use std::{env, io};

use rustyline::error::ReadlineError;
use rustyline::{Editor, Result as RustyResult};

use modules::{ast, error, eval, lexer, parser, token};

fn eval(
    evaluator: &mut eval::Evaluator,
    env: &mut eval::ExprEnv,
    line: &String,
) -> Result<String, error::RispError> {
    let lexer = lexer::Lexer::new(line.clone())?;
    let mut parser = parser::Parser::new(lexer);
    let expr = parser.parse()?;
    let result = evaluator.eval(&expr, env)?;
    Ok(result.to_string())
}

fn main() -> RustyResult<()> {
    let mut evaluator = eval::Evaluator::new();
    let mut env: eval::ExprEnv = eval::default_env();

    if atty::is(atty::Stream::Stdin) {
        let args = env::args().collect::<Vec<String>>();
        if args.len() == 1 {
            let mut rl = Editor::<()>::new()?;
            _ = rl.load_history("history.txt");
            loop {
                let readline = rl.readline("risp>> ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                        if let Some(result) = eval(&mut evaluator, &mut env, &line).ok() {
                            println!("{}", result);
                        } else {
                            continue;
                        }
                    }
                    Err(ReadlineError::Interrupted) => {
                        break;
                    }
                    Err(ReadlineError::Eof) => {
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break;
                    }
                }
            }
            return rl.save_history("history.txt");
        } else {
            let arg = args.get(1);
            if let Some(filename) = arg {
                let file = File::open(filename)?;
                for line in io::BufReader::new(file).lines() {
                    let result = eval(&mut evaluator, &mut env, &line?)
                        .map_err(|_| ReadlineError::Interrupted)?;
                    println!("{}", result);
                }
            }
        }
    } else {
        let stdin = io::stdin();
        for line in stdin.lines() {
            let result =
                eval(&mut evaluator, &mut env, &line?).map_err(|_| ReadlineError::Interrupted)?;
            println!("{}", result);
        }
    }
    Ok(())
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
