use std::io::{self, Read};

const MAX: usize = 100;
const PGMAX: usize = 1001;

#[derive(Clone)]
struct IndexEntry {
    pgnum: [u8; PGMAX], // presence bitmap
    tm: usize,          // occurrence count (increments per input pair)
    str_: String,
}

fn main() {
    // Read all input and parse as whitespace-separated tokens, like scanf("%s %d").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut idx: Vec<IndexEntry> = Vec::new();

    loop {
        let word = match it.next() {
            Some(w) => w,
            None => break,
        };
        let num_tok = match it.next() {
            Some(n) => n,
            None => break,
        };
        let pg_i32: i32 = match num_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // The original C code has undefined behavior if pg is out of range.
        // Under the spec's "valid strings and integers" precondition, pg is expected valid.
        // We defensively ignore out-of-range pages rather than panic/stop.
        if pg_i32 < 0 || (pg_i32 as usize) >= PGMAX {
            continue;
        }
        let pg = pg_i32 as usize;

        if let Some(entry) = idx.iter_mut().find(|e| e.str_ == word) {
            entry.pgnum[pg] = 1;
            entry.tm += 1;
        } else {
            if idx.len() >= MAX {
                // Spec says unique strings won't exceed limit; if it does, stop adding more.
                continue;
            }
            let mut entry = IndexEntry {
                pgnum: [0u8; PGMAX],
                tm: 0,
                str_: word.to_string(),
            };
            entry.pgnum[pg] = 1;
            entry.tm = 1;
            idx.push(entry);
        }
    }

    idx.sort_by(|a, b| a.str_.cmp(&b.str_));

    // Output exactly like the C program:
    // word\n
    // pages... (space-separated) \n
    // Note: if no pages are set (shouldn't happen with valid input), it prints just a blank line.
    let mut out = String::new();
    for entry in &idx {
        out.push_str(&entry.str_);
        out.push('\n');

        let mut printed_any = false;
        for j in 0..PGMAX {
            if entry.pgnum[j] == 1 {
                if printed_any {
                    out.push(' ');
                }
                printed_any = true;
                out.push_str(&j.to_string());
            }
        }
        out.push('\n');
    }

    print!("{out}");
}
