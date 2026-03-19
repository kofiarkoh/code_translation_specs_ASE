use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut count: usize = 0;

    for tok in input.split_whitespace() {
        let v: i64 = tok.parse().unwrap();
        count += 1;

        if v == 0 {
            break;
        }

        println!("Case {}: {}", count, v);
    }
}
