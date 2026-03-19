use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as C's scanf("%d") would for integers separated by whitespace.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // If input is missing, C's behavior would be undefined; we mimic typical contest assumptions.
    let n: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Use i32 with wrapping arithmetic to emulate C signed overflow behavior as closely as possible.
    let mut odd: i32 = 1;
    let mut exp: i32 = 1;

    for _ in 0..n {
        let a: i32 = it.next().unwrap().parse().unwrap();
        if a % 2 == 0 {
            odd = odd.wrapping_mul(2);
        }
    }

    for _ in 1..=n {
        exp = exp.wrapping_mul(3);
    }

    let result = exp.wrapping_sub(odd);
    print!("{}", result);
}
