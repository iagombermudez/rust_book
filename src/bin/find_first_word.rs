// Write a function that takes a string of words
// separated by spaces and returns the first word it finds in that string.
// If the function doesn’t find a space in the string, the whole string
// must be one word, so the entire string should be returned.
fn find_first_word(words: &str) -> &str {
    let bytes = words.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &words[0..i];
        }
    }

    &words[..]
}

fn main() {
    let words = String::from("This is a phrase");
    let first_word = find_first_word(&words);
    println!("{first_word}")
}
