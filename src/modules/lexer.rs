use super::token::Token;

#[derive(Debug)]
pub struct Lexer {
    ch: char,
    input: String,
    length: usize,
    read_position: usize,
    position: usize,
}

type TokenParseError = std::num::ParseFloatError;

impl Lexer {
    pub fn new(input: String) -> Self {
        let length = input.len() - 1;
        let mut lexer = Self {
            ch: '\0',
            input,
            length,
            read_position: 0,
            position: 0,
        };
        lexer.read();
        lexer
    }

    pub fn next_token(&mut self) -> Result<Token, TokenParseError> {
        // Skip if char is whitespace
        while self.ch.is_whitespace() {
            self.read();
        }
        let token = match self.ch {
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '*' => Token::ASTERISK,
            '/' => Token::SLASH,
            '+' => match self.peek().unwrap() {
                '0'..='9' => self.read_as_number()?,
                _ => Token::PLUS,
            },
            '-' => match self.peek().unwrap() {
                '0'..='9' => self.read_as_number()?,
                _ => Token::MINUS,
            },
            '0'..='9' => self.read_as_number()?,
            '"' => self.read_as_string().unwrap(),
            'a'..='z' | 'A'..='Z' => self.read_as_literal().unwrap(),
            '\0' => Token::EOF,
            _ => Token::ILLEGAL(self.ch.to_string()),
        };
        self.read();
        Ok(token)
    }

    fn read_as_literal(&mut self) -> Option<Token> {
        if self.ch == 't' && self.peek()? == '\0' {
            return Some(Token::TRUE);
        }
        let mut s = String::from("");
        loop {
            s.push(self.ch);
            match self.peek()? {
                'a'..='z' | 'A'..='Z' => self.read()?,
                _ => break,
            }
        }
        if s.to_uppercase() == "NIL" {
            return Some(Token::NIL);
        }
        Some(Token::LITERAL(s.to_uppercase()))
    }

    fn read_as_string(&mut self) -> Option<Token> {
        let mut s = String::new();
        loop {
            self.read();
            s.push(self.ch);
            if self.peek()? == '"' {
                self.read();
                break;
            }
        }
        Some(Token::STRING(s))
    }

    fn read_as_number(&mut self) -> Result<Token, TokenParseError> {
        let mut chars = Vec::<char>::new();
        loop {
            chars.push(self.ch);
            if let Some(char) = self.peek() {
                match char {
                    '0'..='9' | '.' => {
                        self.read();
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        let s = chars.iter().collect::<String>();
        let parsed = s.parse::<f64>()?;
        Ok(Token::NUMBER(parsed))
    }

    // increment self.read_position by 1.
    // in case self.read_position gt self.length, finish iteration.
    fn read(&mut self) -> Option<()> {
        self.ch = if self.read_position > self.length {
            '\0'
        } else {
            self.input.chars().nth(self.read_position)?
        };
        self.position = self.read_position;
        self.read_position += 1;
        Some(())
    }

    fn peek(&mut self) -> Option<char> {
        if self.read_position > self.length {
            Some('\0')
        } else {
            self.input.chars().nth(self.read_position)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Token;
    use super::*;

    #[test]
    fn read_test() {
        let mut lexer = Lexer::new("This is test Text".into());
        assert_eq!(lexer.ch, 'T');
        assert_eq!(lexer.length, 16);
        assert_eq!(lexer.position, 0);
        assert_eq!(lexer.read_position, 1);
        lexer.read();
        assert_eq!(lexer.ch, 'h');
        assert_eq!(lexer.position, 1);
        assert_eq!(lexer.read_position, 2);
        lexer.read();
        lexer.read();
        assert_eq!(lexer.ch, 's');
        assert_eq!(lexer.position, 3);
        assert_eq!(lexer.read_position, 4);
    }

    #[test]
    fn peek_test() {
        let input = String::from("peek_test");
        let mut lexer = Lexer::new(input.clone());
        let mut chars = input.chars();
        chars.next();
        while lexer.ch != '\0' {
            if let Some(ch) = chars.next() {
                let char = lexer.peek().unwrap();
                assert_eq!(char, ch);
            }
            lexer.read();
        }
    }

    #[test]
    fn read_invalid_token() {
        let mut lexer = Lexer::new(String::from("^"));
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::ILLEGAL(String::from("^"))
        );
    }

    #[test]
    fn read_string() {
        let mut lexer = Lexer::new(String::from(r#""hello""#));
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::STRING(String::from("hello"))
        );
    }

    #[test]
    fn read_literal() {
        let mut lexer = Lexer::new(String::from("(setq a 2)"));
        assert_eq!(lexer.next_token().unwrap(), Token::LPAREN);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::LITERAL(String::from("SETQ"))
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::LITERAL(String::from("A"))
        );
        assert_eq!(lexer.next_token().unwrap(), Token::NUMBER(2.0));
        assert_eq!(lexer.next_token().unwrap(), Token::RPAREN);
    }

    #[test]
    fn read_var() {
        let mut lexer = Lexer::new(String::from("(+ a 2 a)"));
        assert_eq!(lexer.next_token().unwrap(), Token::LPAREN);
        assert_eq!(lexer.next_token().unwrap(), Token::PLUS);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::LITERAL(String::from("A"))
        );
        assert_eq!(lexer.next_token().unwrap(), Token::NUMBER(2.0));
        assert_eq!(
            lexer.next_token().unwrap(),
            Token::LITERAL(String::from("A"))
        );
        assert_eq!(lexer.next_token().unwrap(), Token::RPAREN);
    }

    #[test]
    fn read_number() {
        let tests = vec![
            ("1", Token::NUMBER(1.0)),
            ("1.5", Token::NUMBER(1.5)),
            ("2.345", Token::NUMBER(2.345)),
        ];
        for test in tests {
            let mut lexer = Lexer::new(test.0.to_string());
            assert_eq!(lexer.next_token().unwrap(), test.1);
        }
    }

    #[test]
    fn basic_arithemetic() {
        let mut lexer = Lexer::new(String::from("(+ 1 2)"));
        assert_eq!(lexer.next_token().unwrap(), Token::LPAREN);
        assert_eq!(lexer.next_token().unwrap(), Token::PLUS);
        assert_eq!(lexer.next_token().unwrap(), Token::NUMBER(1.0));
        assert_eq!(lexer.next_token().unwrap(), Token::NUMBER(2.0));
        assert_eq!(lexer.next_token().unwrap(), Token::RPAREN);
        assert_eq!(lexer.next_token().unwrap(), Token::EOF);
    }

    #[test]
    fn nested_arithmetic() {
        let mut lexer = Lexer::new(String::from("(+ (- 30 2) (* (/ 4 2) 3))"));
        let wants = vec![
            Token::LPAREN,
            Token::PLUS,
            Token::LPAREN,
            Token::MINUS,
            Token::NUMBER(30.0),
            Token::NUMBER(2.0),
            Token::RPAREN,
            Token::LPAREN,
            Token::ASTERISK,
            Token::LPAREN,
            Token::SLASH,
            Token::NUMBER(4.0),
            Token::NUMBER(2.0),
            Token::RPAREN,
            Token::NUMBER(3.0),
            Token::RPAREN,
            Token::RPAREN,
            Token::EOF,
        ];
        for (i, want) in wants.into_iter().enumerate() {
            let token = lexer.next_token().unwrap();
            assert_eq!(
                token, want,
                "unexpected token[{}]: got={:?}, want={:?}",
                i, token, want,
            );
        }
    }
}
