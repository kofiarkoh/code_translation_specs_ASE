use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let n: usize = input.split_whitespace().next().unwrap().parse().unwrap();

    // Fixed-capacity storage for 100 terms as specified.
    let mut terms = [0i64; 100];
    terms[0] = 2;
    terms[1] = 1;

    if n >= 2 {
        for i in 2..=n {
            // Overflow behavior is unspecified by the spec; use normal i64 addition.
            terms[i] = terms[i - 1] + terms[i - 2];
        }
    }

    println!("{}", terms[n]);
}
