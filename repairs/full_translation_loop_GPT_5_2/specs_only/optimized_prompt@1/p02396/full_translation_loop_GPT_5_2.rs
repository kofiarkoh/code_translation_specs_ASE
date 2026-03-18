use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut case_no: i64 = 1;

    for tok in input.split_whitespace() {
        let n: i64 = tok.parse().unwrap(); // Assumes valid integers per specification.
        if n == 0 {
            break;
        }
        println!("Case {}: {}", case_no, n);
        case_no += 1;
    }
}
