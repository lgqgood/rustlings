#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // Wrap the numbers in an Arc to share among threads
    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // Clone the Arc to get a reference for each thread
        let child_numbers = Arc::clone(&shared_numbers);

        let handle = thread::spawn(move || {
            // Sum the numbers with the given offset
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}