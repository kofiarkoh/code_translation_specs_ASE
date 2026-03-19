use std::io::{self, Read};

fn clamp_u8_range(v: i32) -> usize {
    if v < 0 {
        0
    } else if v > 255 {
        255
    } else {
        v as usize
    }
}

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // no more datasets
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let _m: usize = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break,
        };

        if n == 0 {
            break; // termination condition per spec
        }

        let m = _m;

        let mut inc = vec![0i32; m];
        for k in 0..m {
            inc[k] = it.next().unwrap().parse().unwrap();
        }

        // DP over states 0..255; -1 means unreachable (matches C memset(-1)).
        let mut prev = vec![-1i64; 256];
        let mut curr = vec![-1i64; 256];
        prev[128] = 0;

        for _ in 0..n {
            let x: i32 = it.next().unwrap().parse().unwrap();
            curr.fill(-1);

            for s in 0..256 {
                let base = prev[s];
                if base < 0 {
                    continue;
                }
                let s_i32 = s as i32;
                for &a in &inc {
                    let t = clamp_u8_range(s_i32 + a);
                    let diff = x - (t as i32);
                    let cost = (diff as i64) * (diff as i64);
                    let y = base + cost;
                    let slot = &mut curr[t];
                    if *slot < 0 || y < *slot {
                        *slot = y;
                    }
                }
            }

            std::mem::swap(&mut prev, &mut curr);
        }

        // Minimum over all final states.
        let mut res: i64 = (1i64 << 60);
        for &v in &prev {
            if v >= 0 && v < res {
                res = v;
            }
        }

        // C prints as %d (32-bit). Spec allows undefined behavior on overflow;
        // we keep i64 internally but print as integer text.
        out.push_str(&format!("{res}\n"));
    }

    print!("{out}");
}
