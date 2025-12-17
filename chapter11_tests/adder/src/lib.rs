pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn meow() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn meowtwo() {
        let result = add(2, 1);
        assert_eq!(result, 3);
    }
}
