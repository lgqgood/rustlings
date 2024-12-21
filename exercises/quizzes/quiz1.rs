// Function to calculate the price of apples
fn calculate_price_of_apples(quantity: u32) -> u32 {
    // Check if the quantity is greater than 40
    if quantity > 40 {
        // If yes, each apple costs 1 rustbuck
        quantity * 1
    } else {
        // Otherwise, each apple costs 2 rustbucks
        quantity * 2
    }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}