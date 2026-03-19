use std::io::{self, Read};

const MAX: usize = 100;
const PGMAX: usize = 1001;
const WDMAX: usize = 31;

#[derive(Clone)]
struct Index {
    pgnum: [i32; PGMAX],
    tm: i32,
    // C uses a fixed char array and strcpy/strcmp; we store as String but
    // emulate scanf("%s") truncation to WDMAX-1 bytes.
    str_: String,
}

fn swap(idx: &mut [Index; MAX], i: usize, j: usize) {
    // In C, swap copies the whole struct fields via temporary buffers.
    // In Rust, cloning the struct preserves the same effect.
    let tmp = idx[i].clone();
    idx[i] = idx[j].clone();
    idx[j] = tmp;
}

fn main() {
    // Read entire stdin, then tokenize like repeated scanf("%s %d").
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
            None => break, // scanf would fail to read the pair; stop.
        };

        // Emulate scanf("%s") into char tmp_str[WDMAX]:
        // store at most WDMAX-1 bytes (leaving room for '\0' in C).
        let mut tmp_str = word.to_string();
        if tmp_str.as_bytes().len() >= WDMAX {
            tmp_str = String::from_utf8_lossy(&tmp_str.as_bytes()[..(WDMAX - 1)]).into_owned();
        }

        // Parse int similarly to scanf("%d"): if it doesn't parse, stop.
        let tmp_pgnum_i32: i32 = match num_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // C code uses tmp_pgnum as an array index without bounds checks.
        // To preserve behavior without unsafe, we only apply updates when in range.
        // Out-of-range in C would be UB; here we avoid panics.
        let tmp_pgnum = tmp_pgnum_i32 as isize;

        let mut flag = true;
        let mut i = 0usize;
        while i < rgnum {
            if idx[i].str_ == tmp_str {
                if (0..PGMAX as isize).contains(&tmp_pgnum) {
                    idx[i].pgnum[tmp_pgnum as usize] = 1;
                }
                // C int overflow is UB; use wrapping to approximate typical behavior.
                idx[i].tm = idx[i].tm.wrapping_add(1);
                flag = false;
                break;
            }
            i += 1;
        }

        if flag {
            if rgnum < MAX {
                if (0..PGMAX as isize).contains(&tmp_pgnum) {
                    idx[rgnum].pgnum[tmp_pgnum as usize] = 1;
                }
                idx[rgnum].tm = idx[rgnum].tm.wrapping_add(1);
                idx[rgnum].str_ = tmp_str;
                rgnum += 1;
            } else {
                // C would write past idx[MAX]; that's UB. We stop adding new entries.
            }
        }
    }

    // Sort lexicographically by str_ using the same O(n^2) swap logic as C.
    for i in 0..rgnum {
        for j in (i + 1)..rgnum {
            if idx[i].str_ > idx[j].str_ {
                swap(&mut idx, i, j);
            }
        }
    }

    // Output exactly as C: word on its own line, then page numbers separated by
    // spaces, last one followed by '\n'.
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
