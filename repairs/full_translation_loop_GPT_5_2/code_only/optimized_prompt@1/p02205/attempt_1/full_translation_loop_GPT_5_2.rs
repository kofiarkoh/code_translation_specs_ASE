use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut n: i64 = it.next().unwrap().parse().unwrap();
    let mut a: i32 = it.next().unwrap().parse().unwrap();
    let mut b: i32 = it.next().unwrap().parse().unwrap();

    // C: N %= 12; with ll (signed). Rust % matches C's sign behavior for negatives.
    n %= 12;

    // C uses int i and compares i < N (ll). If N is negative, loop never runs.
    // Use i64 for i to mirror the mixed-type comparison without surprises.
    let mut i: i64 = 0;
    while i < n {
        // C int overflow is UB; emulate typical two's-complement wraparound.
        a = a.wrapping_sub(b);
        i += 1;
        if i == n {
            break;
        }
        b = a.wrapping_add(b);
        i += 1;
    }

    // Match C printf("%d %d\n", A, B);
    print!("{} {}\n", a, b);
}
