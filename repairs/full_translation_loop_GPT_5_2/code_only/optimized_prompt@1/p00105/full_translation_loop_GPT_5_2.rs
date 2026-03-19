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

fn main() {
    // In C, idx is uninitialized, but the intended behavior clearly relies on pgnum being 0/1.
    // We zero-initialize to match that intent and to make output deterministic.
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

    // The original C loop is:
    //   while (scanf("%s %d", tmp_str, &tmp_pgnum) != EOF) { ... }
    // For well-formed inputs (pairs of word + int), scanf returns 2 each time.
    // We implement the same effective behavior: process complete pairs only.
    while let (Some(s), Some(pg_s)) = (it.next(), it.next()) {
        // C "%s" into char[WDMAX] can overflow; we approximate by truncating to WDMAX-1.
        let tmp_str: String = s.chars().take(WDMAX - 1).collect();

        // If the integer token isn't a valid i32, C scanf would stop assigning and the loop
        // would not be well-defined; we stop to avoid inventing behavior.
        let tmp_pgnum: i32 = match pg_s.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        let mut flag = true;

        for i in 0..rgnum {
            if idx[i].str_ == tmp_str {
                if (0..PGMAX as i32).contains(&tmp_pgnum) {
                    idx[i].pgnum[tmp_pgnum as usize] = 1;
                }
                idx[i].tm = idx[i].tm.wrapping_add(1);
                flag = false;
                break;
            }
        }

        if flag {
            if rgnum >= MAX {
                // C would write out of bounds; stop to avoid memory corruption.
                break;
            }
            if (0..PGMAX as i32).contains(&tmp_pgnum) {
                idx[rgnum].pgnum[tmp_pgnum as usize] = 1;
            }
            idx[rgnum].tm = idx[rgnum].tm.wrapping_add(1);
            idx[rgnum].str_ = tmp_str;
            rgnum += 1;
        }
    }

    // Match C's final ordering: lexicographic by string (strcmp).
    idx[..rgnum].sort_by(|a, b| a.str_.cmp(&b.str_));

    // Output formatting must match C exactly.
    let mut out = String::new();
    for i in 0..rgnum {
        out.push_str(&idx[i].str_);
        out.push('\n');

        let mut k: i32 = 0;
        for j in 0..PGMAX {
            if idx[i].pgnum[j] == 1 {
                if k == idx[i].tm - 1 {
                    out.push_str(&format!("{j}\n"));
                    break;
                } else {
                    out.push_str(&format!("{j} "));
                    k += 1;
                }
            }
        }
    }

    print!("{out}");
}
