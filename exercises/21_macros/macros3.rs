// Use the #[macro_use] attribute to bring the macros module into scope
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    // Now you can call the macro since it is brought into scope
    my_macro!();
}