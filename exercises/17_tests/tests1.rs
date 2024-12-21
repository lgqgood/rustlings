fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // Import everything from the outer module.
    use super::*;

    #[test]
    fn you_can_assert() {
        // Test the function `is_even` with some values.
        assert!(is_even(2));  // 2 is even, should return true
        assert!(!is_even(3)); // 3 is odd, should return false
        assert!(is_even(0));  // 0 is even, should return true
        assert!(is_even(-4)); // -4 is even, should return true
        assert!(!is_even(-5)); // -5 is odd, should return false
    }
}