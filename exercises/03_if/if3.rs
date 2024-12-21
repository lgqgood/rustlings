fn animal_habitat(animal: &str) -> &str {
    // TODO: Fix the compiler error in the statement below.
    let identifier = if animal == "crab" {
        "Beach"
    } else if animal == "gopher" {
        "Burrow"
    } else if animal == "snake" {
        "Desert"
    } else {
        "Unknown"
    };

    // Don't change the expression below!
    if identifier == "Beach" {
        "Beach"
    } else if identifier == "Burrow" {
        "Burrow"
    } else if identifier == "Desert" {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    // You can optionally experiment here.
    println!("A crab lives in {}", animal_habitat("crab"));
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
