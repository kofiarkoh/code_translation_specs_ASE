use std::io::{self, Read};

const PGMAX: usize = 1001; // pages 0..=1000

#[derive(Clone)]
struct Entry {
    word: String,
    pg: [u8; PGMAX], // presence table; "present" iff value == 1
    tm: i32,         // counter incremented once per input record for this word
}

fn main() {
    // Tokenize stdin similarly to repeated scanf("%s %d"):
    // consume pairs; stop when a token is missing or page parse fails.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut entries: Vec<Entry> = Vec::new();

    loop {
        let w = match it.next() {
            Some(s) => s.to_string(),
            None => break,
        };
        let p_tok = match it.next() {
            Some(s) => s,
            None => break,
        };
        let p_i32 = match p_tok.parse::<i32>() {
            Ok(v) => v,
            Err(_) => break,
        };

        // The C spec says out-of-range page numbers cause UB (OOB).
        // In Rust, avoid panics; ignore such records (narrow, non-inventive behavior).
        let p = match usize::try_from(p_i32).ok().filter(|&u| u < PGMAX) {
            Some(u) => u,
            None => continue,
        };

        if let Some(e) = entries.iter_mut().find(|e| e.word == w) {
            e.pg[p] = 1;
            e.tm = e.tm.wrapping_add(1);
        } else {
            let mut e = Entry {
                word: w,
                pg: [0u8; PGMAX],
                tm: 0,
            };
            e.pg[p] = 1;
            e.tm = e.tm.wrapping_add(1);
            entries.push(e);
        }
    }

    // Sort by ascending lexicographic order of the word strings.
    // Use byte-wise ordering to better match C strcmp on typical inputs.
    entries.sort_by(|a, b| a.word.as_bytes().cmp(b.word.as_bytes()));

    let mut out = String::new();

    for e in &entries {
        out.push_str(&e.word);
        out.push('\n');

        // Print page indices j where pg[j] == 1, scanning j increasing.
        // Stop (and print newline) only when exactly tm such indices have been printed.
        // If tm is larger than the number of present pages, no terminating newline is printed.
        let mut printed: i32 = 0;
        for j in 0..PGMAX {
            if e.pg[j] == 1 {
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
