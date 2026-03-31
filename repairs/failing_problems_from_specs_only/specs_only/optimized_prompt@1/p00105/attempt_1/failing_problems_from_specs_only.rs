use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, Read};

const MAX: usize = 30;
const WDMAX: usize = 20;
const PGMAX: usize = 1001; // indices 0..=1000

#[derive(Clone)]
struct Entry {
    word: String,
    // In the C spec these are not explicitly initialized (indeterminate).
    // Rust cannot represent indeterminate memory safely; we choose the narrowest
    // reasonable, deterministic interpretation: zero-initialized.
    pgnum: [u8; PGMAX],
    tm: usize,
}

fn main() {
    // Read entire stdin and parse as whitespace-delimited tokens, like scanf("%s %d").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    let mut entries: Vec<Entry> = Vec::new();
    let mut index: HashMap<String, usize> = HashMap::new();

    loop {
        let w = match it.next() {
            Some(s) => s,
            None => break,
        };
        let p_tok = match it.next() {
            Some(s) => s,
            None => break, // incomplete trailing record: stop like failed scanf
        };
        let p_i32: i32 = match p_tok.parse() {
            Ok(v) => v,
            Err(_) => break, // parse failure: stop like failed scanf
        };

        // Spec: word token must fit in WDMAX including NUL; overflow is UB in C.
        // We do not emulate UB; we accept the token as-is.
        let word = w.to_string();

        // Spec: page number used directly as index; out-of-range is UB in C.
        // We choose the narrowest safe behavior: ignore out-of-range indices.
        if p_i32 < 0 || (p_i32 as usize) >= PGMAX {
            continue;
        }
        let p = p_i32 as usize;

        if let Some(&idx) = index.get(&word) {
            let e = &mut entries[idx];
            e.pgnum[p] = 1;
            e.tm = e.tm.wrapping_add(1);
        } else {
            if entries.len() >= MAX {
                // Spec: exceeding MAX causes out-of-bounds writes / UB in C.
                // We avoid UB and stop accepting new distinct words.
                continue;
            }
            let mut e = Entry {
                word: word.clone(),
                pgnum: [0u8; PGMAX],
                tm: 0,
            };
            e.pgnum[p] = 1;
            e.tm = e.tm.wrapping_add(1);
            entries.push(e);
            index.insert(word, entries.len() - 1);
        }
    }

    // Sort by ascending lexicographic order of word strings.
    entries.sort_by(|a, b| {
        // strcmp-like bytewise lexicographic order; Rust's String Ord is lexicographic by Unicode scalar values.
        // Given the spec's C context, inputs are expected to be plain bytes/ASCII; this is the closest safe match.
        a.word.cmp(&b.word)
    });

    // Output.
    let mut out = String::new();
    for e in &entries {
        out.push_str(&e.word);
        out.push('\n');

        // Print page indices j where pgnum[j] == 1, scanning 0..PGMAX-1.
        // Stop when exactly tm such indices have been printed.
        // If tm exceeds the number of 1s encountered, no terminating newline is printed for the page list.
        let mut printed = 0usize;
        for j in 0..PGMAX {
            if e.pgnum[j] == 1 {
                printed = printed.wrapping_add(1);
                if printed == e.tm {
                    out.push_str(&format!("{j}\n"));
                    break;
                } else {
                    out.push_str(&format!("{j} "));
                }
            }
        }
    }

    print!("{out}");
}
