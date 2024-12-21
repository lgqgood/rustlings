macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // Fixed macro call
    my_macro!();
}