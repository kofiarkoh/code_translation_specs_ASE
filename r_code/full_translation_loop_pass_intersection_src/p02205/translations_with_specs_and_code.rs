use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Per spec: exactly three integers are provided; otherwise output is unspecified.
    let n: i64 = it.next().unwrap().parse().unwrap();
    let mut a: i32 = it.next().unwrap().parse().unwrap();
    let mut b: i32 = it.next().unwrap().parse().unwrap();

    // Spec: step count is non-negative; k = N mod 12 in [0, 11].
    // C uses N %= 12 on signed; with the precondition N >= 0, this matches.
    let k: i64 = n % 12;

    // Apply exactly k steps:
    // odd t:  (A, B) -> (A - B, B)
    // even t: (A, B) -> (A, A + B)
    for t in 1..=k {
        if t % 2 == 1 {
            a = a - b;
        } else {
            b = a + b;
        }
    }

    // Preserve output formatting: "A B\n"
    println!("{} {}", a, b);
}
