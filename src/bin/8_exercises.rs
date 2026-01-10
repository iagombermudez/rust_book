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

    // Convert strings to Pig Latin. The first consonant of each word is moved to
    // the end of the word and ay is added, so first becomes irst-fay.
    // Words that start with a vowel have hay added to the end instead
    // (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
    //

    fn ip_latinpay_ordway(word: &mut String) -> String {
        let first_char = word.remove(0);
        let vowels = "aeiouAEIOU";
        if vowels.contains(first_char) {
            word.insert(0, first_char);
            word.push_str("-hay");
        } else {
            word.push_str("-");
            word.push(first_char);
            word.push_str("ay");
        }
        word.to_string()
    }

    let mut word = String::from("first");
    let piglatin = ip_latinpay_ordway(&mut word);
    println!("{piglatin}");

    let mut word = String::from("apple");
    let piglatin = ip_latinpay_ordway(&mut word);
    println!("{piglatin}");
}
