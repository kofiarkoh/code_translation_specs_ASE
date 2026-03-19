use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens (matches scanf behavior for %d).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: i32 = it.next().unwrap().parse().unwrap();

    let n_usize = n as usize;
    let m_usize = m as usize;

    // C uses VLAs: int s[N], c[M]; Here we use Vecs.
    // Note: In the original C, s[0]/c[0] are read even when M==0? No, but later s[0]/c[0]
    // are still accessed unconditionally, which is UB. We emulate a common "zero-initialized"
    // outcome by initializing with 0s so behavior is deterministic in Rust.
    let mut s: Vec<i32> = vec![0; n_usize];
    let mut c: Vec<i32> = vec![0; m_usize];

    let d: [i32; 3] = [100, 10, 1];
    let mut digit: Vec<i32> = vec![-1; n_usize];

    let mut num: i32 = 0;

    if m == 0 {
        match n {
            1 => num = 0,
            2 => num = 10,
            3 => num = 100,
            _ => {}
        }
    } else {
        for i in 0..m_usize {
            let si: i32 = it.next().unwrap().parse().unwrap();
            let ci: i32 = it.next().unwrap().parse().unwrap();
            if i < s.len() {
                s[i] = si;
            }
            c[i] = ci;

            let idx = (si - 1) as usize;
            let di = 3 - n + (si - 1); // index into d[]
            let dval = d[di as usize];

            if digit[idx] == -1 {
                digit[idx] = ci * dval;
            } else if digit[idx] / dval != ci {
                num = -1;
            }
        }

        if num != -1 {
            for i in 0..n_usize {
                if digit[i] != -1 {
                    num += digit[i];
                } else if n > 1 && i == 0 && digit[i] != 0 {
                    // This condition is always true when digit[i] == -1, matching the C code.
                    let di = 3 - n + (i as i32);
                    num += d[di as usize];
                }
            }
        }
    }

    // Matches the C's final special-case check.
    if n > 1 && s.get(0).copied().unwrap_or(0) == 1 && c.get(0).copied().unwrap_or(0) == 0 {
        print!("-1\n"); // puts adds newline
    } else {
        print!("{}", num); // printf("%d") without newline
    }
}
