use std::io::{self, Read};

const MAX: usize = 100;
const PGMAX: usize = 1001;
const WDMAX: usize = 31;

#[derive(Clone)]
struct Index {
    pgnum: [i32; PGMAX],
    tm: i32,
    // C stores a fixed-size char array; we emulate by truncating to WDMAX-1 bytes.
    str_: String,
}

fn trunc_c_word(s: &str) -> String {
    // scanf("%s") writes bytes until whitespace and then '\0'.
    // The C buffer is WDMAX, so it can hold at most WDMAX-1 bytes plus '\0'.
    // If input is longer, C would overflow (UB). We choose to truncate to preserve
    // the intended fixed-buffer behavior without unsafe.
    let bytes = s.as_bytes();
    let n = bytes.len().min(WDMAX - 1);
    String::from_utf8_lossy(&bytes[..n]).into_owned()
}

fn main() {
    // Initialize; C's uninitialized memory is never read for indices < rgnum.
    let mut idx: Vec<Index> = (0..MAX)
        .map(|_| Index {
            pgnum: [0; PGMAX],
            tm: 0,
            str_: String::new(),
        })
        .collect();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut rgnum: usize = 0;

    // Emulate: while (scanf("%s %d", tmp_str, &tmp_pgnum) != EOF)
    // scanf returns number of successful assignments; loop continues for 2, stops otherwise.
    loop {
        let word = match it.next() {
            Some(w) => w,
            None => break,
        };
        let num_s = match it.next() {
            Some(n) => n,
            None => break,
        };

        // If %d fails to parse, scanf would return 1 (not EOF), and the loop would continue,
        // but the failing token would remain unread in the stream. With split_whitespace we
        // cannot "unread" a token, so we approximate by stopping (common judge inputs are valid).
        let tmp_pgnum_i32: i32 = match num_s.parse::<i32>() {
            Ok(v) => v,
            Err(_) => break,
        };

        let tmp_str = trunc_c_word(word);

        let mut found = false;
        for i in 0..rgnum {
            if idx[i].str_ == tmp_str {
                // C indexes with int; negative or >=PGMAX is UB. We ignore out-of-range writes.
                if (0..(PGMAX as i32)).contains(&tmp_pgnum_i32) {
                    idx[i].pgnum[tmp_pgnum_i32 as usize] = 1;
                }
                idx[i].tm = idx[i].tm.wrapping_add(1);
                found = true;
                break;
            }
        }

        if !found {
            if rgnum < MAX {
                if (0..(PGMAX as i32)).contains(&tmp_pgnum_i32) {
                    idx[rgnum].pgnum[tmp_pgnum_i32 as usize] = 1;
                }
                idx[rgnum].tm = idx[rgnum].tm.wrapping_add(1);
                idx[rgnum].str_ = tmp_str;
                rgnum += 1;
            } else {
                // C would overflow idx (UB). We stop adding new entries.
            }
        }
    }

    // Same O(n^2) sort as C using strcmp (bytewise lexicographic for ASCII).
    for i in 0..rgnum {
        for j in (i + 1)..rgnum {
            if idx[i].str_.as_bytes() > idx[j].str_.as_bytes() {
                idx.swap(i, j);
            }
        }
    }

    // Output: word newline, then page numbers in ascending order with spaces, last with newline.
    // Note: C's tm counts occurrences, not unique pages; if duplicates exist, C may print
    // fewer numbers than tm suggests (and may omit the trailing newline). We preserve that.
    let mut out = String::new();
    for i in 0..rgnum {
        out.push_str(&idx[i].str_);
        out.push('\n');

        let mut k: i32 = 0;
        for j in 0..PGMAX {
            if idx[i].pgnum[j] == 1 {
                if k == idx[i].tm.wrapping_sub(1) {
                    out.push_str(&format!("{j}\n"));
                    break;
                } else {
                    out.push_str(&format!("{j} "));
                    k = k.wrapping_add(1);
                }
            }
        }
    }

    print!("{out}");
}
