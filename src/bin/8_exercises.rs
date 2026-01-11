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

    // Using a hash map and vectors, create a text interface to allow
    // a user to add employee names to a department in a company; for
    // example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then, let the user retrieve a list of all people in a department
    // or all people in the company by department, sorted alphabetically.
    struct Company {
        departments: HashMap<String, Vec<String>>,
    }

    impl Company {
        pub fn new() -> Self {
            Self {
                departments: HashMap::new(),
            }
        }
        fn add_employee(&mut self, name: &str, department: &str) {
            let people = self
                .departments
                .entry(department.to_string())
                .or_insert_with(Vec::new);
            people.push(name.to_string());
        }

        fn get_by_department(&self, department: &str) {
            let people = self.departments.get(department);
            match people {
                Some(people) => {
                    println!("People in department {department}");
                    for person in people {
                        println!("{person}");
                    }
                }
                None => println!("Department {department} does not exist"),
            }
        }

        fn get_all(&self) {
            for (department, _) in &self.departments {
                self.get_by_department(&department[..]);
            }
        }
    }

    let mut company = Company::new();
    company.add_employee("Sally", "Finance");
    company.add_employee("Cory", "Finance");
    company.add_employee("Anthony", "Business");
    company.add_employee("Mark", "Finance");
    company.get_by_department("Finance");
    company.get_by_department("Business");
    company.get_by_department("Marketing");
    company.get_all();
}
