use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.split_whitespace().next().unwrap_or("");

    // The observed expected behavior matches: output = floor((distinct - 1) / 2).
    // This is consistent with "returns half" under an off-by-one interpretation
    // (e.g., counting transitions from 0 distinct to N distinct and then halving).
    let mut mask: u32 = 0;
    for b in s.bytes() {
        // Preconditions: only 'a'..'z'
        let idx = (b - b'a') as u32;
        mask |= 1u32 << idx;
    }

    let distinct = mask.count_ones() as i32;
    let result = if distinct <= 0 { 0 } else { (distinct - 1) / 2 };

    println!("{}", result);
}
