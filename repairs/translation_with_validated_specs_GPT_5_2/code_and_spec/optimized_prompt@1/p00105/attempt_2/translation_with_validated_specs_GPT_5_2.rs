use std::io::{self, Read};

const MAX: usize = 100;
const PGMAX: usize = 1001;
const WDMAX: usize = 31;

#[derive(Clone)]
struct IndexEntry {
    pgnum: [i32; PGMAX],
    tm: i32,
    str_: String,
}

fn main() {
    // Read stdin fully; tokenize like scanf("%s %d") (whitespace-delimited).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut idx: Vec<IndexEntry> = Vec::new();

    while let Some(w_tok) = it.next() {
        let p_tok = match it.next() {
            Some(s) => s,
            None => break, // incomplete record => scanf would fail, stop
        };
        let p: i32 = match p_tok.parse() {
            Ok(v) => v,
            Err(_) => break, // scanf would fail, stop
        };

        // C uses fixed buffer WDMAX and would overflow on longer tokens (UB).
        // To approximate the intended behavior under valid inputs, we truncate to WDMAX-1.
        let w = if w_tok.len() >= WDMAX {
            &w_tok[..WDMAX - 1]
        } else {
            w_tok
        };

        let mut found = false;
        for entry in idx.iter_mut() {
            if entry.str_ == w {
                // Spec: out-of-range page index is UB in C. Here we ignore it to stay safe.
                if (0..PGMAX as i32).contains(&p) {
                    entry.pgnum[p as usize] = 1;
                }
                entry.tm += 1;
                found = true;
                break;
            }
        }

        if !found {
            if idx.len() >= MAX {
                // Exceeding MAX is UB in C; keep behavior defined by dropping extra entries.
                continue;
            }
            let mut entry = IndexEntry {
                pgnum: [0; PGMAX],
                tm: 0,
                str_: w.to_string(),
            };
            if (0..PGMAX as i32).contains(&p) {
                entry.pgnum[p as usize] = 1;
            }
            entry.tm += 1;
            idx.push(entry);
        }
    }

    // Sort ascending lexicographic order.
    idx.sort_by(|a, b| a.str_.cmp(&b.str_));

    // Emit output with the same spacing/newline rules as the C code.
    let mut out = String::new();
    for entry in &idx {
        out.push_str(&entry.str_);
        out.push('\n');

        let mut k: i32 = 0;
        for j in 0..PGMAX {
            if entry.pgnum[j] == 1 {
                if k == entry.tm - 1 {
                    out.push_str(&j.to_string());
                    out.push('\n');
                    break;
                } else {
                    out.push_str(&j.to_string());
                    out.push(' ');
                    k += 1;
                }
            }
        }
        // If tm is larger than the number of present pages encountered, no newline is forced,
        // matching the specified behavior.
    }

    print!("{out}");
}
