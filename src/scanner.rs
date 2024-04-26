use crate::errors::CalculatorError;
use crate::token::Token;
use crate::token::TokenType;




#[derive(Debug)]
pub struct Scanner<'a> {
    input : &'a str,
    current : usize,
    pub token_list : Vec<Token>,
}




impl std::error::Error for CalculatorError {}

impl<'a> Scanner<'a> {

    pub fn new(input: &'a str) -> Scanner {
        return Scanner {
             input: input,
             current: 0,
             token_list: Vec::new(),
        };
    }

    fn is_eof(&self) -> bool {
        return self.current >= self.input.len();
    }

    fn peek(&self) -> Result<char, CalculatorError> {
        return self.input.chars().nth(self.current).ok_or(CalculatorError::EndOfFile());
    }

    fn next(&mut self) -> Result<char, CalculatorError> {
        self.current += 1;
        return self.peek();
    }

    fn add_token(&mut self, t_type : TokenType, value : Option<f64>) {
        self.token_list.push(Token {t_type, value })
    }

    pub fn parse(&mut self) -> Result<(), CalculatorError> {
        loop {
            if self.is_eof() {
                break;
            }
            let mut peek_char = self.peek()?;
            match peek_char {
                ' ' | '\n' | '\t' | '\r' => self.current += 1,
                '+' | '-' | '*' | '/' | '(' | ')' => {
                    self.add_token(TokenType::from_char(peek_char)?, None);
                    self.current += 1;
                }
                _ => {
                    if peek_char.is_digit(10) {
                        let mut index : i32 = 0;
                        let mut number : f64 = 0.0;

                        while peek_char.is_digit(10) {
                            let tmp : f64 = peek_char.to_digit(10).unwrap().into();
                            number = number * 10.0 + tmp;
                            index += 1;

                            // next
                            peek_char = self.next()?;
                        }

                        if peek_char == '.' {
                            index = -1;

                            // next
                            peek_char = self.next()?;

                            while peek_char.is_digit(10) {
                                let tmp : f64 = peek_char.to_digit(10).unwrap().into();
                                number += 10.0_f64.powi(index) * tmp;
                                index -= 1;

                                // next
                                peek_char = self.next()?;
                            }

                        }

                        self.add_token(TokenType::Number, Option::Some(number))

                    } else {
                        return Err(CalculatorError::InvalidCharacter(peek_char, self.current));
                    }
                }
            }
        } 
        Ok(())
    }

}