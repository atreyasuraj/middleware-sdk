
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum ArithmeticError {
    #[error("Integer overflow on an operation with {a} and {b}")]
    IntegerOverflow { a: u64, b: u64 },
}

#[uniffi::export]
pub fn add(a: u64, b: u64) -> Result<u64> {
    a.checked_add(b)
        .ok_or(ArithmeticError::IntegerOverflow { a, b })
}

#[uniffi::export]
fn sub(a: u64, b: u64) -> Result<u64> {
    a.checked_sub(b)
        .ok_or(ArithmeticError::IntegerOverflow { a, b })
}

#[uniffi::export]
fn div(dividend: u64, divisor: u64) -> u64 {
    if divisor == 0 {
        panic!("Can't divide by zero");
    }
    dividend / divisor
}

#[uniffi::export]
fn equal(a: u64, b: u64) -> bool {
    a == b
}

type Result<T, E = ArithmeticError> = std::result::Result<T, E>;
