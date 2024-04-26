use std::fmt::{self, write};

use crate::token::TokenType;

#[derive(Debug)]
pub enum CalculatorError {
    OperatorNotAvailable(char),

    InvalidCharacter(char, usize),

    EndOfFile(),

    ClosingParenthesisNotFound(usize, TokenType),
    NumberExpected(usize, TokenType),
}


impl fmt::Display for CalculatorError{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CalculatorError::InvalidCharacter(ch, idx) => {
                write!(f, "Invalid character '{}' at index {}", ch, idx)
            }
            CalculatorError::OperatorNotAvailable(ch) => {
                write!(f, "Operator not available for the character '{}'", ch)
            }
            CalculatorError::EndOfFile() => {
                write!(f, "End of file unexpected error")
            }
            CalculatorError::ClosingParenthesisNotFound(index, t_type) => {
                write!(f,  "Closing parenthesis expected at '{}' index but found '{:?}'.", index, t_type)
            }

            CalculatorError::NumberExpected(index, t_type) => {
                write!(f,  "Number expected at '{}' index but found '{:?}'.", index, t_type)
            }
           
        }
    }
}