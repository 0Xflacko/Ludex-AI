use crate::error::ArithmeticError;

pub fn get_amounts_out(amount_in: u64, reserve_in: u64, reserve_out: u64) -> Result<u64, ArithmeticError> {
    if reserve_in == 0 || reserve_out == 0 {
        return Err(ArithmeticError::ZeroReserve);
    }

    // Convert to u128 for intermediate calculations to avoid overflow
    let amount_in_u128 = amount_in as u128;
    let reserve_in_u128 = reserve_in as u128;
    let reserve_out_u128 = reserve_out as u128;

    // Calculate amount out using constant product formula: (x + Δx) * (y - Δy) = x * y
    // Solving for Δy: Δy = y - (x * y) / (x + Δx)
    
    // Calculate numerator: amount_in * reserve_out
    let numerator = amount_in_u128.checked_mul(reserve_out_u128).ok_or(ArithmeticError::Overflow)?;
    
    // Calculate denominator: reserve_in + amount_in
    let denominator = reserve_in_u128.checked_add(amount_in_u128).ok_or(ArithmeticError::Overflow)?;
    
    // Calculate result: numerator / denominator
    let result_u128 = numerator.checked_div(denominator).ok_or(ArithmeticError::DivisionByZero)?;
    
    // Check if result fits in u64
    if result_u128 > u64::MAX as u128 {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(result_u128 as u64)
}   