use crate::errors::CalculatorError;

#[derive(Debug)]
pub struct Token {
    t_type: TokenType,
    value : Option<f64>
}

#[derive(Debug)]
pub enum TokenType {
    Number,
    Plus,
    Minus,
    Cross,
    Division,

    LParen,
    RParen
}

impl TokenType {
    fn from_char(item: char) -> Result<Self, CalculatorError> {
        
        match item {
            '+' => Ok(TokenType::Plus),
            '-' => Ok(TokenType::Minus),
            '*' => Ok(TokenType::Cross),
            '/' => Ok(TokenType::Division),
            '(' => Ok(TokenType::LParen),
            ')' => Ok(TokenType::RParen),
            _ => Err(CalculatorError::OperatorNotAvailable(item)),
        }
    }
}

