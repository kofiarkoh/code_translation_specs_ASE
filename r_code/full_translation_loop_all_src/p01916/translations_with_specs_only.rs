use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let token = input.split_whitespace().next().unwrap();

    // Track parity (even/odd) for each letter 'a'..'z' using a 26-bit mask.
    // Bit i is 1 iff the count of ('a' + i) seen so far is odd.
    let mut parity_mask: u32 = 0;

    for b in token.bytes() {
        // Preconditions guarantee lowercase 'a'..'z'; otherwise behavior is undefined.
        let idx = (b - b'a') as u32;
        parity_mask ^= 1u32 << idx;
    }

    let odd_distinct = parity_mask.count_ones();
    let result = odd_distinct / 2;

    print!("{}", result);
}
