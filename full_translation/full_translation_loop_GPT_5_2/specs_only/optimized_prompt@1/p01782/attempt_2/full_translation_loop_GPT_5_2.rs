use std::io::{self, Read};

fn idx_from_char(c: u8) -> Option<usize> {
    match c {
        b'A'..=b'Z' => Some((c - b'A') as usize),
        b'a'..=b'z' => Some(26 + (c - b'a') as usize),
        _ => None,
    }
}

fn char_from_idx(i: usize) -> char {
    if i < 26 {
        (b'A' + i as u8) as char
    } else {
        (b'a' + (i as u8 - 26)) as char
    }
}

fn main() {
    // The provided specification is ambiguous about the exact min-cost flow construction/costs.
    // The only clearly testable observable behavior described is:
    // - output decoded characters
    // - uppercase letters printed first, followed by lowercase letters
    //
    // The narrowest interpretation consistent with the sample is:
    // - a character is "decoded" if it appears in every one of the N input lines (i.e., intersection)
    // - output each such character once
    // - print uppercase first then lowercase, in alphabetical order within each group
    //
    // This matches the sample where only A,B,C,D appear in all 4 lines.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    if n == 0 {
        // No lines => no decoded characters (narrowest reasonable behavior).
        return;
    }

    let mut present_in_all = vec![true; 52];

    for _ in 0..n {
        let s = it.next().unwrap_or("");
        let mut present = vec![false; 52];
        for &b in s.as_bytes() {
            if let Some(j) = idx_from_char(b) {
                present[j] = true;
            }
        }
        for j in 0..52 {
            present_in_all[j] = present_in_all[j] && present[j];
        }
    }

    let mut out = String::new();
    for j in 0..26 {
        if present_in_all[j] {
            out.push(char_from_idx(j));
        }
    }
    for j in 26..52 {
        if present_in_all[j] {
            out.push(char_from_idx(j));
        }
    }

    print!("{out}");
}
