use std::io::{self, Read};

fn f(mut n: i64, mut x: i64) -> i64 {
    // Iterative form to match the specified linear nesting depth without recursion limits.
    let mut acc: i64 = 0;

    while n > 0 {
        // Preconditions state 2^n is representable in the signed integer range,
        // so shifting is safe for the intended inputs.
        let two_pow_n: i64 = 1i64 << (n as u32);
        let p: i64 = two_pow_n - 1;
        let two_p: i64 = 2 * p;

        if x <= two_p {
            x = x - 1;
        } else {
            acc += p + 1;
            x = x - 1 - two_p;
        }

        n -= 1;
    }

    // Base case: F(0, x) = 0 if x <= 0 else 1
    if x <= 0 { acc } else { acc + 1 }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    let result = f(n, k);
    print!("{result}");
}
