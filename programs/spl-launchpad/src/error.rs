use anchor_lang::prelude::*;

#[error_code]
pub enum ArithmeticError {
    #[msg("Division by zero")]
    DivisionByZero,
    #[msg("Overflow")]
    Overflow,
    #[msg("Zero reserve")]
    ZeroReserve,
}