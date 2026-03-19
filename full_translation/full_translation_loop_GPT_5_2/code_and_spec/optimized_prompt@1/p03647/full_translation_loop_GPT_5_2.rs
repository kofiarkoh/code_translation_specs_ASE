use std::io::{self, Read};

fn main() {
    // Read all input tokens.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let toks: Vec<i32> = s
        .split_whitespace()
        .map(|t| t.parse::<i32>().unwrap())
        .collect();

    if toks.is_empty() {
        return;
    }

    // The original C does: scanf("%d%*d",&n) (reads n and discards one int),
    // but many real inputs for this task provide only one header integer (n).
    // To preserve intended behavior across both formats, choose the start offset
    // that makes the remaining token count even (pairs).
    let n = toks[0];
    let start = if toks.len() >= 2 && (toks.len() - 2) % 2 == 0 {
        2 // header had two ints: n and a discarded one
    } else {
        1 // header had only n
    };

    const SZ: usize = 1 << 18;
    let mut m = vec![0i32; SZ];
    let mut n_flag = n;

    let mut i = start;
    while i + 1 < toks.len() {
        let a = toks[i];
        let b = toks[i + 1];
        i += 2;

        // p = m + (a-1 ? (b-n ? 0 : a) : b);
        let idx_i32 = if a != 1 {
            if b != n_flag { 0 } else { a }
        } else {
            b
        };
        let idx = idx_i32 as usize;

        // Update step: p-m && *p ? n=0 : ++*p
        if idx != 0 && m[idx] != 0 {
            n_flag = 0;
        } else {
            m[idx] = m[idx].wrapping_add(1);
        }
    }

    if n_flag != 0 {
        print!("IMPOSSIBLE\n");
    } else {
        print!("POSSIBLE\n");
    }
}
