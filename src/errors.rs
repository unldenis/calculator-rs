use std::fmt::{self, write};

#[derive(Debug)]
pub enum CalculatorError {
    OperatorNotAvailable(char),

    InvalidCharacter(char, usize),

    EndOfFile()
}


impl fmt::Display for CalculatorError {

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
        }
    }
}