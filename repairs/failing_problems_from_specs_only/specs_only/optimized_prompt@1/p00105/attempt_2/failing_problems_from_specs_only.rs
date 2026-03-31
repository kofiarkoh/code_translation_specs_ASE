use std::collections::HashMap;
use std::io::{self, Read};

const MAX: usize = 30;
const PGMAX: usize = 1001; // 0..=1000

#[derive(Clone)]
struct Entry {
    word: String,
    // C version has indeterminate initial contents; Rust cannot model that safely.
    // We use zero-initialization as the narrowest deterministic behavior.
    pgnum: [u8; PGMAX],
    tm: usize,
}

fn main() {
    // scanf("%s %d")-like tokenization: whitespace-delimited.
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
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
            None => break,
        };
        let p_i32: i32 = match p_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // C would have UB for out-of-range page indices; we avoid UB by ignoring.
        if p_i32 < 0 || (p_i32 as usize) >= PGMAX {
            continue;
        }
        let p = p_i32 as usize;

        // Word identity: exact string equality (strcmp).
        // Use owned String keys to avoid lifetime issues.
        let word = w.to_string();

        if let Some(&idx) = index.get(&word) {
            let e = &mut entries[idx];
            e.pgnum[p] = 1;
            e.tm = e.tm.wrapping_add(1);
        } else {
            // C would overflow fixed arrays if >MAX; we avoid UB by ignoring new words.
            if entries.len() >= MAX {
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

    // Sort by ascending lexicographic order of the word strings.
    // Use bytewise ordering to better match strcmp semantics.
    entries.sort_by(|a, b| a.word.as_bytes().cmp(b.word.as_bytes()));

    // Stream output directly to avoid large intermediate allocations.
    let mut out = String::new();
    for e in &entries {
        out.push_str(&e.word);
        out.push('\n');

        let mut printed = 0usize;
        for j in 0..PGMAX {
            if e.pgnum[j] == 1 {
                printed = printed.wrapping_add(1);
                if printed == e.tm {
                    out.push_str(&j.to_string());
                    out.push('\n');
                    break;
                } else {
                    out.push_str(&j.to_string());
                    out.push(' ');
                }
            }
        }
    }

    print!("{out}");
}
