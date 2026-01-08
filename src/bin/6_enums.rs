struct Example {
    test: String,
}
enum IpAddr {
    V4(Example),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    //let v4 = IpAddr::V4(Example {
    //    test: String::from("Hola"),
    //});

    //let some_number = Some(5);
    //let some_char = Some('e');
    //let absent_number: Option<i32> = None;
    //let config_max = Some(3u8);
    //if let Some(max) = config_max {
    //    println!("The maximum is configured to be {max}")
    //}
    //
    //

    fn describe_state__quarter(coin: Coin) -> Option<String> {
        let Coin::Quarter(state) = coin else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!(
                "{state:?} is relatively new.
    "
            ))
        }
    }
}
