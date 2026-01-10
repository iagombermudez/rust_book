use std::collections::HashMap;

fn main() {
    // Given a list of integers, use a vector and return the median
    // (when sorted, the value in the middle position) and mode
    // (the value that occurs most often; a hash map will be helpful here)
    // of the list.
    println!("Exercise 1");
    let mut integers = vec![1, 2, 3, 1, 1, 2, 4, 5, 6, 7, 7, 8, 9, 10];
    integers.sort();
    let median = integers[integers.len() / 2];
    println!("Median: {median}");
    let mut counts = HashMap::new();
    for int in integers.iter() {
        let count = counts.entry(int).or_insert(0);
        *count += 1;
    }
    // The following lambda deconstructs the value so we can compare
    // by value instead of key
    let mode = counts.iter().max_by_key(|(_, value)| *value);
    match mode {
        Some(mode) => {
            let mode_key = mode.0;
            let mode_value = mode.1;
            println!("Mode: {mode_key} -> {mode_value}")
        }
        None => println!("No mode found"),
    }
}
