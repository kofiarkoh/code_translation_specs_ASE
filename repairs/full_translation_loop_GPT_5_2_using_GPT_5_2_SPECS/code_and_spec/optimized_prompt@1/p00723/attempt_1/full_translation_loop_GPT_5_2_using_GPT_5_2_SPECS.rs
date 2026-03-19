use std::collections::HashSet;
use std::io::{self, Read};

fn reverse_range(bytes: &mut [u8], start: usize, len: usize) {
    let mut i = 0usize;
    while i < len / 2 {
        bytes.swap(start + i, start + (len - 1 - i));
        i += 1;
    }
}

/// Mimics the C `swap(s1, s2)` where `s2` points inside `s1`:
/// result becomes `s2..end` followed by `s1..s2`.
fn rotate_at(bytes: &mut [u8], split: usize) {
    // Equivalent to left-rotating by `split`: [0..split) moves to the end.
    bytes.rotate_left(split);
}

fn main() {
    // Read all input, whitespace-delimited.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut out = String::new();

    for _ in 0..t {
        let s = it.next().unwrap_or("");
        let orig = s.as_bytes().to_vec();
        let len = orig.len();

        // SPEC: If L <= 1, output 0.
        if len <= 1 {
            out.push_str("0\n");
            continue;
        }

        // SPEC bounds: <= 1024 distinct strings; we use a HashSet for uniqueness.
        let mut set: HashSet<Vec<u8>> = HashSet::new();

        for i in 1..len {
            // First sequence: starting from original, apply reversals in fixed pattern.
            let mut cur = orig.clone();

            set.insert(cur.clone());
            reverse_range(&mut cur, 0, i);
            set.insert(cur.clone());
            reverse_range(&mut cur, i, len - i);
            set.insert(cur.clone());
            reverse_range(&mut cur, 0, i);
            set.insert(cur.clone());

            // Second sequence: swap (rotate) around split, then apply reversals in fixed pattern.
            cur = orig.clone();
            rotate_at(&mut cur, i);

            set.insert(cur.clone());
            reverse_range(&mut cur, 0, len - i);
            set.insert(cur.clone());
            reverse_range(&mut cur, len - i, i);
            set.insert(cur.clone());
            reverse_range(&mut cur, 0, len - i);
            set.insert(cur.clone());
        }

        out.push_str(&format!("{}\n", set.len()));
    }

    print!("{out}");
}
