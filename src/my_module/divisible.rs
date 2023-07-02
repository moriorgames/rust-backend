pub fn is_not_divisible_by_three(num: u32) -> bool {
    num % 3 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_not_divisible_by_three() {
        assert_eq!(is_not_divisible_by_three(4), true);
        assert_eq!(is_not_divisible_by_three(9), false);
        assert_eq!(is_not_divisible_by_three(15), false);
    }
}
