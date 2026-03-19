use std::io::{self, Read};

const MAX: usize = 100;
const PGMAX: usize = 1001;
const WDMAX: usize = 31;

#[derive(Clone)]
struct Index {
    pgnum: [i32; PGMAX],
    tm: i32,
    str_: String,
}

fn swap(idx: &mut [Index; MAX], i: usize, j: usize) {
    let tmp = idx[i].clone();
    idx[i] = idx[j].clone();
    idx[j] = tmp;
}

fn main() {
    // Read all input and split like scanf("%s %d") would across whitespace.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut idx: [Index; MAX] = std::array::from_fn(|_| Index {
        pgnum: [0; PGMAX],
        tm: 0,
        str_: String::new(),
    });

    let mut rgnum: usize = 0;

    loop {
        let word = match it.next() {
            Some(w) => w,
            None => break,
        };
        let num_tok = match it.next() {
            Some(n) => n,
            None => break,
        };

        // Emulate scanf("%s") into char tmp_str[WDMAX]:
        // truncate to WDMAX-1 bytes.
        let bytes = word.as_bytes();
        let bytes = if bytes.len() >= WDMAX {
            &bytes[..(WDMAX - 1)]
        } else {
            bytes
        };
        let tmp_str = String::from_utf8_lossy(bytes).into_owned();

        let tmp_pgnum: i32 = match num_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        let mut flag = true;

        // Search existing entries.
        for i in 0..rgnum {
            if idx[i].str_ == tmp_str {
                // C would index without bounds checks (UB if out of range).
                // We avoid panics; for in-range indices we match behavior.
                if (0..PGMAX as i32).contains(&tmp_pgnum) {
                    idx[i].pgnum[tmp_pgnum as usize] = 1;
                }
                idx[i].tm = idx[i].tm.wrapping_add(1);
                flag = false;
                break;
            }
        }

        if flag {
            if rgnum < MAX {
                if (0..PGMAX as i32).contains(&tmp_pgnum) {
                    idx[rgnum].pgnum[tmp_pgnum as usize] = 1;
                }
                idx[rgnum].tm = idx[rgnum].tm.wrapping_add(1);
                idx[rgnum].str_ = tmp_str;
                rgnum += 1;
            } else {
                // C would overflow idx; we stop adding new entries.
            }
        }
    }

    // Sort by strcmp (lexicographic).
    for i in 0..rgnum {
        for j in (i + 1)..rgnum {
            if idx[i].str_ > idx[j].str_ {
                swap(&mut idx, i, j);
            }
        }
    }

    // Print exactly like the C code.
    // Important: if tm is larger than the number of set pages (duplicates),
    // the C code prints all pages with trailing spaces and *no* terminating '\n'
    // for that word. We preserve that behavior.
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
