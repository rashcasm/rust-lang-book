//! # My Crate
//!
//! `my_crate` is a small example crate used to demonstrate
//! how Rust documentation comments (`///` and `//!`) work.
//!
//! ## Features
//! - Simple arithmetic functions
//! - Well-documented APIs
//! - Runnable examples
//!
//! ## Getting Started
//!
//! ```
//! use my_crate::add_one;
//!
//! let x = 10;
//! assert_eq!(add_one(x), 11);
//! ```

/// Adds one to the number given.
///
/// This function takes an integer and returns the value increased by one.
///
/// # Parameters
///
/// - `x`: The integer to increment.
///
/// # Returns
///
/// The value of `x + 1`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use my_crate::add_one;
///
/// let arg = 5;
/// let answer = add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
/// Using a negative number:
///
/// ```
/// use my_crate::add_one;
///
/// assert_eq!(add_one(-1), 0);
/// ```
///
/// # Notes
///
/// - This function does **not** mutate the input.
/// - It performs no overflow checks in release mode.
///
/// # See Also
///
/// - [`add_two`]
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Adds two to the number given.
///
/// This is implemented by calling [`add_one`] twice.
///
/// # Examples
///
/// ```
/// use my_crate::add_two;
///
/// assert_eq!(add_two(3), 5);
/// ```
pub fn add_two(x: i32) -> i32 {
    add_one(add_one(x))
}

/// Adds a custom value to a number.
///
/// # Panics
///
/// This function will panic if `value` is negative.
///
/// # Examples
///
/// ```
/// use my_crate::add_custom;
///
/// assert_eq!(add_custom(10, 5), 15);
/// ```
pub fn add_custom(x: i32, value: i32) -> i32 {
    if value < 0 {
        panic!("value must be non-negative");
    }
    x + value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(1), 2);
    }

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(2), 4);
    }
}
