use std::io::{self, Read};

const MAX: usize = 100;
const PGMAX: usize = 1001;
const WDMAX: usize = 31;

#[derive(Clone)]
struct Index {
    pgnum: [i32; PGMAX],
    tm: i32,
    // C: char str[WDMAX] (NUL-terminated). We store the visible prefix (up to first NUL).
    str_: String,
}

fn trunc_c_word(s: &str) -> String {
    // Emulate storing into `char tmp_str[WDMAX]` via scanf("%s"):
    // keep at most WDMAX-1 bytes (leaving room for '\0').
    let b = s.as_bytes();
    let n = b.len().min(WDMAX - 1);
    String::from_utf8_lossy(&b[..n]).into_owned()
}

fn main() {
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

    // C loop: while (scanf("%s %d", tmp_str, &tmp_pgnum) != EOF)
    // For well-formed judge inputs, this corresponds to consuming pairs of tokens.
    while let (Some(word), Some(num_s)) = (it.next(), it.next()) {
        let tmp_str = trunc_c_word(word);

        // Assume valid integer tokens (typical for problems using scanf).
        // If invalid, C would not consume the token and could loop; we stop to avoid divergence.
        let tmp_pgnum: i32 = match num_s.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        let mut flag = true;
        let mut i = 0usize;
        while i < rgnum {
            if idx[i].str_ == tmp_str {
                if (0..PGMAX as i32).contains(&tmp_pgnum) {
                    idx[i].pgnum[tmp_pgnum as usize] = 1;
                }
                idx[i].tm = idx[i].tm.wrapping_add(1);
                flag = false;
                break;
            }
            i += 1;
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
                // C would overflow idx (UB). We ignore additional distinct words.
            }
        }
    }

    // Sort by strcmp: bytewise lexicographic on the stored (truncated) bytes.
    for i in 0..rgnum {
        for j in (i + 1)..rgnum {
            if idx[i].str_.as_bytes() > idx[j].str_.as_bytes() {
                idx.swap(i, j);
            }
        }
    }

    // Output exactly like C: word newline, then pages in ascending order.
    // Important: C prints a newline only when it reaches the (tm-1)-th printed page.
    // If tm exceeds the number of unique pages, C prints trailing spaces and no newline.
    let mut out = String::new();
    for i in 0..rgnum {
        out.push_str(&idx[i].str_);
        out.push('\n');

        let mut k: i32 = 0;
        for j in 0..PGMAX {
            if idx[i].pgnum[j] == 1 {
                if k == idx[i].tm.wrapping_sub(1) {
                    out.push_str(&j.to_string());
                    out.push('\n');
                    break;
                } else {
                    out.push_str(&j.to_string());
                    out.push(' ');
                    k = k.wrapping_add(1);
                }
            }
        }
    }

    print!("{out}");
}
