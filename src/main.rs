use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    for word in "Hello this is a literal hello this".split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
