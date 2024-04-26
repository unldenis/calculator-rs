use std::cell::Cell;

use crate::errors::CalculatorError;
use crate::token::Token;
use crate::token::TokenType;

pub struct Parser<'a> {
    token_list : &'a Vec<Token>,
    index : Cell<usize>
}

impl<'a> Parser<'a>  {

    pub fn new(token_list : &'a Vec<Token>) -> Parser<'a> {
        Parser { token_list, index: Cell::new(0) }
    }

    fn current(&self) -> Result<&Token, CalculatorError> {
        return self.token_list.get(self.index.get()).ok_or(CalculatorError::EndOfFile());
    }

    /* 
    private fun eat(): Token {
        val t = current()
        index++
        return t
    }
    */
    fn eat(&self) -> Result<&Token, CalculatorError> {
        let current = self.current();
        self.index.set(self.index.get() + 1);

        return current;
    }

    fn parse_primary(&self) -> Result<f64, CalculatorError> {

        if(self.current()?.t_type == TokenType::LParen) {
            // lparen
            self.eat()?;

            let paren = self.parse_term()?;
            if(self.current()?.t_type != TokenType::RParen) {
                return Err(CalculatorError::ClosingParenthesisNotFound(self.index.get(), self.current()?.t_type))
            }

            //RParen
            self.eat()?;

            // return paren
            return Ok(paren);
        }
        let t = self.eat()?;
        if(t.t_type != TokenType::Number) {
            return Err(CalculatorError::NumberExpected(self.index.get(), t.t_type));
        }
        
        Ok(t.value.unwrap())
    }

    fn parse_unary(&self) -> Result<f64, CalculatorError> {
        if(self.current()?.t_type == TokenType::Minus) {
            // minus
            self.eat()?;

            let right = self.parse_primary()?;
            return Ok(-right);
        }
        return self.parse_primary();
    }

    fn parse_factor(&self) -> Result<f64, CalculatorError> {
        let mut left = self.parse_unary()?;
        while self.index.get() < self.token_list.len() && (self.current()?.t_type == TokenType::Cross || self.current()?.t_type == TokenType::Division) {
            let op = self.eat()?.t_type;
            let right = self.parse_unary()?;

            left *= if(op == TokenType::Cross) { right } else {1.0 / right}
        }
        return Ok(left);
    }

    fn parse_term(&self) -> Result<f64, CalculatorError> {
        let mut left = self.parse_factor()?;
        while self.index.get() < self.token_list.len() && (self.current()?.t_type == TokenType::Plus || self.current()?.t_type == TokenType::Minus) {
            let op = self.eat()?.t_type;
            let right = self.parse_factor()?;
            
            left += if(op == TokenType::Plus) { right } else {-right}
        }
        return Ok(left);
    }

    pub fn parse(&self) -> Result<f64, CalculatorError> {
        return self.parse_term();
    }

}