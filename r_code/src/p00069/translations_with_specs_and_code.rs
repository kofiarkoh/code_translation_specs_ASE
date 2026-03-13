// amida.rs
//
// Translated from the provided C implementation following the SPEC.
// Behavior: For each dataset, determine whether the Amida (ladder) mapping
// already places value `m` at column `atari`. If not, search for a single
// horizontal rung insertion (row-major order) that achieves it, respecting
// adjacency constraints. Outputs "0", "r c", or "1" per dataset.
//
// This implementation follows the specification exactly and preserves the
// traversal semantics. It uses safe, idiomatic Rust (no unsafe).

use std::io::{self, Read};

fn check_amida(
    amida: &Vec<Vec<u8>>, // '0' or '1' bytes
    temp: &mut [usize],   // length n, 1-based labels stored
    n: usize,
    d: usize,
    atari: usize, // 1-based
    m: usize,
) -> bool {
    // Simulate traversal top-to-bottom, swapping adjacent labels where '1' present.
    for k in 0..d {
        for l in 0..(n.saturating_sub(1)) {
            if amida[k][l] == b'1' {
                temp.swap(l, l + 1);
            }
        }
    }
    temp[atari - 1] == m
}

fn ins_amida(
    amida: &mut Vec<Vec<u8>>,
    a: usize,
    b: usize,
    temp: &mut [usize],
    n: usize,
    d: usize,
    atari: usize,
    m: usize,
) -> bool {
    // Insert rung, check, then remove rung (restoring original state).
    amida[a][b] = b'1';
    let res = check_amida(amida, temp, n, d, atari, m);
    amida[a][b] = b'0';
    res
}

fn main() {
    // Read entire stdin and split by whitespace (matches scanf/token behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut it = input.split_whitespace();

    // Process datasets until n == 0 or input exhausted.
    loop {
        let n_token = match it.next() {
            Some(tok) => tok,
            None => break,
        };
        let n: usize = match n_token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        if n == 0 {
            break;
        }

        // Read m, atari, d
        let m: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        let atari: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };
        let d: usize = match it.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => break,
        };

        // Read d row strings
        let mut amida: Vec<Vec<u8>> = Vec::with_capacity(d);
        for _ in 0..d {
            let row = match it.next() {
                Some(s) => s.as_bytes().to_vec(),
                None => Vec::new(),
            };
            amida.push(row);
        }

        // Initialize keep and temp arrays with labels 1..=n
        let keep: Vec<usize> = (1..=n).collect();
        let mut temp: Vec<usize> = keep.clone();

        let mut flag = false;

        // Initial check without any insertion
        if check_amida(&amida, &mut temp, n, d, atari, m) {
            println!("0");
            flag = true;
        }

        if !flag {
            'outer: for i in 0..d {
                for j in 0..(n.saturating_sub(1)) {
                    // Reset temp before each candidate attempt (matches C behavior)
                    temp.copy_from_slice(&keep);

                    // Skip if there's already a rung here
                    if amida[i].get(j).copied().unwrap_or(b'0') == b'1' {
                        continue;
                    }
                    // Skip if adjacent rung to the left in same row
                    if j > 0 {
                        if amida[i].get(j - 1).copied().unwrap_or(b'0') == b'1' {
                            continue;
                        }
                    }
                    // Skip if adjacent rung to the right in same row
                    if j != n.saturating_sub(2) {
                        if amida[i].get(j + 1).copied().unwrap_or(b'0') == b'1' {
                            continue;
                        }
                    }

                    // If a candidate, attempt insertion
                    if ins_amida(&mut amida, i, j, &mut temp, n, d, atari, m) {
                        // Output 1-based row and column indices
                        println!("{} {}", i + 1, j + 1);
                        flag = true;
                        break 'outer;
                    }
                }
            }
        }

        if !flag {
            println!("1");
        }
    }
}
