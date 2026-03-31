use std::cmp::Ordering;
use std::io::{self, Read};

const PGMAX: usize = 1001; // indices 0..=1000
// MAX and WDMAX exist in the C program but are only relevant for UB constraints there.
// In Rust we do not impose those limits unless specified by observable behavior.

#[derive(Clone)]
struct Entry {
    word: String,
    pg: Vec<u8>, // page-presence table; values compared to 1
    tm: i64,     // per-word counter incremented per input record
}

fn main() {
    // Read entire stdin and tokenize like scanf("%s %d") repeatedly.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut entries: Vec<Entry> = Vec::new();

    loop {
        let w = match it.next() {
            Some(s) => s.to_string(),
            None => break,
        };
        let p_str = match it.next() {
            Some(s) => s,
            None => break, // incomplete trailing record: stop like failed scanf
        };
        let p_i64 = match p_str.parse::<i64>() {
            Ok(v) => v,
            Err(_) => break, // parse failure: stop like failed scanf
        };

        // The C spec says out-of-range page numbers cause UB via OOB access.
        // In Rust, we choose the narrowest reasonable behavior: ignore records
        // whose page index cannot be represented in 0..PGMAX-1, rather than panic.
        let p_usize = match usize::try_from(p_i64).ok().filter(|&u| u < PGMAX) {
            Some(u) => u,
            None => continue,
        };

        // Find existing entry by exact string equality (strcmp == 0).
        if let Some(e) = entries.iter_mut().find(|e| e.word == w) {
            e.pg[p_usize] = 1;
            e.tm = e.tm.wrapping_add(1);
        } else {
            let mut e = Entry {
                word: w,
                pg: vec![0u8; PGMAX],
                tm: 0,
            };
            e.pg[p_usize] = 1;
            e.tm = e.tm.wrapping_add(1);
            entries.push(e);
        }
    }

    // Sort ascending lexicographic order of word strings.
    entries.sort_by(|a, b| {
        // Match C strcmp ordering on bytes for ASCII; for general UTF-8, Rust's
        // String Ord is lexicographic by Unicode scalar values. The spec only
        // states "lexicographic order" without encoding details; use Rust's Ord.
        a.word.cmp(&b.word)
    });

    let mut out = String::new();

    for e in &entries {
        out.push_str(&e.word);
        out.push('\n');

        // Printing semantics:
        // Scan j = 0..PGMAX-1; for each pg[j] == 1, print j.
        // Print spaces after each printed number except the one considered "last".
        // "Last" is when exactly tm indices with pg[j]==1 have been printed.
        // If tm exceeds the number of such indices encountered, no terminating newline
        // is printed for the page list (only the word line's newline exists).
        let mut printed: i64 = 0;
        for j in 0..PGMAX {
            if e.pg[j] == 1 {
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
