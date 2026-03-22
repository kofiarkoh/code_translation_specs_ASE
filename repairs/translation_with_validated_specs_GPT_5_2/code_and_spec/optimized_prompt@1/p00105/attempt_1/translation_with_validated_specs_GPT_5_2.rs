use std::io::{self, Read};

const MAX: usize = 100;
const PGMAX: usize = 1001;
const WDMAX: usize = 31;

#[derive(Clone)]
struct IndexEntry {
    // SPEC note: In the C program these are NOT explicitly initialized, so their
    // initial values are indeterminate (undefined behavior in C). In Rust we must
    // choose defined values; we initialize to 0, which is the closest sensible intent.
    pgnum: [i32; PGMAX],
    tm: i32,
    str_: String,
}

fn main() {
    // Read entire stdin, then parse as whitespace-delimited tokens like scanf("%s %d").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut idx: Vec<IndexEntry> = Vec::new();

    loop {
        let w = match it.next() {
            Some(s) => s,
            None => break,
        };
        let p_tok = match it.next() {
            Some(s) => s,
            None => break, // incomplete record at EOF: stop (scanf would fail)
        };
        let p: i32 = match p_tok.parse() {
            Ok(v) => v,
            Err(_) => break, // scanf would fail; spec only defines normal inputs
        };

        // Enforce WDMAX buffer behavior only insofar as we must store a string.
        // The C code would overflow for longer tokens (UB). Here we keep the full token.
        // (Spec: longer tokens are UB in C; Rust defines behavior.)
        let mut found = false;

        for entry in idx.iter_mut() {
            if entry.str_ == w {
                // Spec: page number used directly as index; out-of-range is UB in C.
                // In Rust, we avoid panics by checking; if out of range, we do nothing,
                // which is a defined deviation from C's UB.
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
                // Spec: exceeding MAX is UB in C. We stop adding further entries to avoid panic.
                // No error message per spec.
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

    // Sort ascending lexicographic order by word string.
    idx.sort_by(|a, b| a.str_.cmp(&b.str_));

    // Output formatting must match C: word line, then page numbers with spaces/newline rules.
    let mut out = String::new();
    for entry in &idx {
        out.push_str(&entry.str_);
        out.push('\n');

        let mut k: i32 = 0;
        for j in 0..PGMAX {
            if entry.pgnum[j] == 1 {
                if k == entry.tm - 1 {
                    out.push_str(&format!("{j}\n"));
                    break;
                } else {
                    out.push_str(&format!("{j} "));
                    k += 1;
                }
            }
        }
        // SPEC: If tm exceeds the number of present pages encountered, C may print no
        // terminating newline for the page list. We preserve that behavior by not forcing
        // a newline here.
    }

    print!("{out}");
}
