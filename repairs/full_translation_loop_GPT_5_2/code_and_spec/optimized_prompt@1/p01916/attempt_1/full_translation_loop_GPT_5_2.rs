use std::io::{self, Read};

fn main() {
    // Read entire stdin, then take the first whitespace-delimited token (like scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.split_whitespace().next().unwrap_or("");

    // SPEC says: count distinct lowercase letters and output half that count.
    // Note: The provided C code actually toggles parity per character occurrence (XOR),
    // which counts letters with odd frequency, not distinct letters. We follow the SPEC.
    let mut seen = [false; 26];
    for b in s.bytes() {
        let idx = (b - b'a') as usize; // precondition: 'a'..='z'
        seen[idx] = true;
    }

    let distinct = seen.iter().filter(|&&v| v).count() as i32;
    println!("{}", distinct / 2);
}
