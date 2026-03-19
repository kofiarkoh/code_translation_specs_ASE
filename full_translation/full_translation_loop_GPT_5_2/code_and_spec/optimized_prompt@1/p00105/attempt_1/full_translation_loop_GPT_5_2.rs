use std::io::{self, Read};

const MAX: usize = 100;
const PGMAX: usize = 1001;

#[derive(Clone)]
struct IndexEntry {
    // In the C code this is an int array used as a presence bitmap (0/1) indexed by page number.
    pgnum: [u8; PGMAX],
    // Count of how many times the string was seen (C increments even for duplicate page numbers).
    tm: usize,
    str_: String,
}

fn main() {
    // Read entire stdin, then parse as whitespace-separated tokens (matches scanf("%s %d") behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut idx: Vec<IndexEntry> = Vec::new();

    loop {
        let s = match it.next() {
            Some(v) => v,
            None => break,
        };
        let n_tok = match it.next() {
            Some(v) => v,
            None => break, // incomplete pair; spec doesn't define behavior, so stop.
        };
        let pg: usize = match n_tok.parse() {
            Ok(v) => v,
            Err(_) => break, // spec says valid integers; if not, stop.
        };

        // The C code assumes pg is within [0, PGMAX). We preserve that precondition.
        if pg >= PGMAX {
            // Spec doesn't define error handling; avoid panicking and just stop processing.
            break;
        }

        // Find existing entry.
        if let Some(entry) = idx.iter_mut().find(|e| e.str_ == s) {
            entry.pgnum[pg] = 1;
            entry.tm += 1; // note: increments even if pg was already present (matches C)
        } else {
            // Respect MAX unique strings limit (precondition in spec).
            if idx.len() >= MAX {
                break;
            }
            let mut entry = IndexEntry {
                pgnum: [0u8; PGMAX],
                tm: 0,
                str_: s.to_string(),
            };
            entry.pgnum[pg] = 1;
            entry.tm += 1;
            idx.push(entry);
        }
    }

    // Sort lexicographically by string (matches strcmp-based sort).
    idx.sort_by(|a, b| a.str_.cmp(&b.str_));

    // Output:
    // C prints the string on its own line, then prints associated page numbers on the next line,
    // space-separated, with a trailing newline.
    // Note: C's `tm` may exceed the number of unique page numbers; it uses `tm` only to decide
    // when to print the last number without a trailing space, but also breaks when it reaches
    // the end of the bitmap. We replicate the observable output by printing all present pages
    // in ascending order, space-separated, then newline.
    let mut out = String::new();
    for entry in &idx {
        out.push_str(&entry.str_);
        out.push('\n');

        let mut first = true;
        for (j, &v) in entry.pgnum.iter().enumerate() {
            if v == 1 {
                if !first {
                    out.push(' ');
                }
                first = false;
                out.push_str(&j.to_string());
            }
        }
        out.push('\n');
    }

    print!("{out}");
}
