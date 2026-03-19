use std::io::{self, Read};

fn digit_sum(mut x: i32) -> i32 {
    // Spec requires digit-sum consistency; implement generally (not limited to 5 digits like C).
    let mut s = 0;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    s
}

fn main() {
    // Read all input; behavior is undefined on invalid integers per spec, so we unwrap parsing.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();

    // Note: C code uses 5-digit extraction, which would be incorrect for N >= 100000.
    // The spec defines digit sum for each integer, so we follow the spec with a general digit_sum.
    let mut count: i32 = 0;
    for i in 1..=n {
        let s = digit_sum(i);
        if a <= s && s <= b {
            count += i;
        }
    }

    print!("{}", count);
}
