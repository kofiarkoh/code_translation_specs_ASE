use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse one integer n.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: usize = input.split_whitespace().next().unwrap().parse().unwrap();

    // C code uses `ll l[100];` but the spec allows 0 <= n <= 100 and mentions
    // storage for up to 101 values. We follow the spec and allocate 101 slots.
    // Note: This is a mismatch with the C array size (100), which would be
    // out-of-bounds for n == 100.
    let mut l = [0i64; 101];

    l[0] = 2;
    l[1] = 1;
    for i in 2..=n {
        l[i] = l[i - 2] + l[i - 1];
    }

    print!("{}\n", l[n]);
}
