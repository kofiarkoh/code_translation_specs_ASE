use std::collections::HashSet;
use std::io::{self, Read};

fn reverse_prefix(buf: &mut [u8], n: usize) {
    buf[..n].reverse();
}

fn reverse_suffix_from(buf: &mut [u8], start: usize) {
    buf[start..].reverse();
}

fn rotate_prefix_to_end(t: &[u8], i: usize) -> Vec<u8> {
    // s := t[i..] + t[..i]
    let mut out = Vec::with_capacity(t.len());
    out.extend_from_slice(&t[i..]);
    out.extend_from_slice(&t[..i]);
    out
}

fn main() {
    // Read all input tokens (whitespace-delimited), matching scanf("%d") / scanf("%s") behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut outputs = String::new();

    for _ in 0..n {
        let s0 = it.next().unwrap();
        let t = s0.as_bytes().to_vec();
        let len = t.len();

        // Per spec: if L <= 1, no split positions exist => 0 candidates.
        if len <= 1 {
            outputs.push_str("0\n");
            continue;
        }

        // The C program uses a fixed array a[1024][80] and linear contain();
        // semantically it's a per-test-case set of distinct strings.
        let mut set: HashSet<Vec<u8>> = HashSet::new();

        for i in 1..len {
            // A. Reversal-based sequence on split (i, len-i)
            let mut s = t.clone();
            set.insert(s.clone());

            reverse_prefix(&mut s, i);
            set.insert(s.clone());

            reverse_suffix_from(&mut s, i);
            set.insert(s.clone());

            reverse_prefix(&mut s, i);
            set.insert(s.clone());

            // B. Rotation-then-reversal sequence
            let mut s = rotate_prefix_to_end(&t, i);
            set.insert(s.clone());

            let left_len = len - i; // prefix length to reverse in this sequence
            reverse_prefix(&mut s, left_len);
            set.insert(s.clone());

            // reverse suffix of length i => reverse from index left_len
            reverse_suffix_from(&mut s, left_len);
            set.insert(s.clone());

            reverse_prefix(&mut s, left_len);
            set.insert(s.clone());
        }

        outputs.push_str(&format!("{}\n", set.len()));
    }

    print!("{outputs}");
}
