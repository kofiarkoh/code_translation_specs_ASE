use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse three integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();

    // Per spec, behavior is only well-defined for positive a, b, c and valid rank.
    // We mirror the C logic: iterate downward from min(a,b), count common divisors,
    // stop when the c-th is found, and print it.
    let mut sum: i32 = 0;
    let mut l: i32 = 0; // In C, l is uninitialized if preconditions fail; spec allows undefined.
                         // Here we initialize to 0 to keep Rust defined, but under valid inputs
                         // it will be overwritten before printing.

    let start = if a > b { b } else { a };

    for i in (1..=start).rev() {
        if a % i == 0 && b % i == 0 {
            sum += 1;
        }
        if sum == c {
            l = i;
            break;
        }
    }

    print!("{l}\n");
}
