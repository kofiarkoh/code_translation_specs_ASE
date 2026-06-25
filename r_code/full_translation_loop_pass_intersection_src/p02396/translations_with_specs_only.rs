use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut case_counter: i64 = 0;

    for tok in input.split_whitespace() {
        let value: i64 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break, // unspecified by spec; stop on malformed token
        };

        case_counter += 1;

        if value == 0 {
            break;
        }

        println!("Case {}: {}", case_counter, value);
    }
}
