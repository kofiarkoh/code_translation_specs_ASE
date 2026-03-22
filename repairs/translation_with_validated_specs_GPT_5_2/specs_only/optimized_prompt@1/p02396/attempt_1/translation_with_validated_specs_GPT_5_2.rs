use std::io::{self, Read};

fn main() {
    // Read entire stdin. The specification does not define behavior on malformed input or EOF;
    // we keep behavior narrow and simple: if parsing fails or input ends before a zero appears,
    // the program will stop producing further output (unspecified by spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut case_counter: i64 = 0;

    for tok in input.split_whitespace() {
        let value: i64 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break, // unspecified; choose to stop
        };

        case_counter += 1;

        if value == 0 {
            break;
        }

        // Labeled line with 1-based read position and the value.
        // The exact label text is not specified; keep it minimal and unambiguous.
        println!("{} {}", case_counter, value);
    }
}
