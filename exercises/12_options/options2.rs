fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // Use if-let to extract the value from the Option
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // Use while-let to handle the nested Option structure
        while let Some(integer) = optional_integers.pop() {
            if let Some(value) = integer {
                assert_eq!(value, cursor as i8);
                cursor -= 1;
            }
        }

        assert_eq!(cursor, 0);
    }
}