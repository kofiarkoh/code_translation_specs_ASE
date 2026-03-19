use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as C's scanf("%d") would for integers separated by whitespace.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // In the C code, failing scanf would leave variables uninitialized (UB),
    // but we assume valid input as typical for such tasks.
    let n: i32 = it.next().unwrap().parse().unwrap();

    // Use i32 with wrapping arithmetic to emulate C signed overflow behavior
    // as it would occur on typical two's-complement machines.
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
