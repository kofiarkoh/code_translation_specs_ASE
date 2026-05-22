use std::io::{self, Read};

fn atoi_like_c(s: &[u8]) -> i32 {
    // Emulate C's atoi: skip leading whitespace, parse optional sign and digits,
    // stop at first non-digit, return 0 if no digits.
    let mut i = 0usize;
    while i < s.len() && s[i].is_ascii_whitespace() {
        i += 1;
    }

    let mut sign: i32 = 1;
    if i < s.len() {
        if s[i] == b'+' {
            i += 1;
        } else if s[i] == b'-' {
            sign = -1;
            i += 1;
        }
    }

    let mut any = false;
    let mut val: i32 = 0;
    while i < s.len() {
        let c = s[i];
        if !c.is_ascii_digit() {
            break;
        }
        any = true;
        let d = (c - b'0') as i32;
        // C's atoi has undefined behavior on overflow; we use wrapping to keep
        // deterministic behavior and mimic typical two's-complement wraparound.
        val = val.wrapping_mul(10).wrapping_add(d);
        i += 1;
    }

    if !any {
        0
    } else {
        val.wrapping_mul(sign)
    }
}

fn func(s: &[u8], n: usize, m: i32) -> i32 {
    // C version temporarily writes NULs into the string and calls atoi on substrings.
    // Here we avoid mutation by slicing at the same boundaries.
    let mut sum: i32 = 0;
    let mut p_start: usize = 0;

    for i in 0..n {
        if (m & (1i32 << i)) != 0 {
            // In C: char t=s[i+1]; s[i+1]=0; sum+=atoi(p); s[i+1]=t; p=s+i+1;
            // This implies i+1 is within the string buffer; in the original loop
            // i ranges 0..n-1 effectively for meaningful splits, but we preserve
            // the exact condition check.
            let end = (i + 1).min(s.len());
            sum = sum.wrapping_add(atoi_like_c(&s[p_start..end]));
            p_start = end;
        }
    }

    sum.wrapping_add(atoi_like_c(&s[p_start..]))
}

fn main() {
    // Read entire stdin and tokenize like scanf("%d%s", &target, paper)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let target_opt = it.next();
        if target_opt.is_none() {
            break;
        }
        let target: i32 = match target_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let paper_opt = it.next();
        if paper_opt.is_none() {
            break;
        }
        let paper_str = paper_opt.unwrap();

        if target == 0 {
            break;
        }

        let paper_bytes = paper_str.as_bytes();
        let n = paper_bytes.len();

        let mut res: i32 = 0;
        let mut best: i32 = -1;
        let mut cnt: i32 = 0;

        // for(i=(1<<(n-1));i<(1<<n);++i)
        // Note: In C, shifting by >= width is undefined; we assume inputs keep n reasonable.
        let start = 1i32 << ((n as i32) - 1);
        let end = 1i32 << (n as i32);

        let mut i = start;
        while i < end {
            let t = func(paper_bytes, n, i);
            if t <= target {
                if best < t {
                    best = t;
                    res = i;
                    cnt = 1;
                } else if best == t {
                    cnt += 1;
                }
            }
            i += 1;
        }

        if cnt != 1 {
            if cnt == 0 {
                out.push_str("error\n");
            } else {
                out.push_str("rejected\n");
            }
        } else {
            out.push_str(&format!("{} ", best));
            for idx in 0..n {
                out.push(paper_bytes[idx] as char);
                if (res & (1i32 << (idx as i32))) != 0 {
                    if idx == n - 1 {
                        out.push('\n');
                    } else {
                        out.push(' ');
                    }
                }
            }
        }
    }

    print!("{out}");
}
