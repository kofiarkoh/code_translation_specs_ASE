use std::io::{self, Read};

fn atoi_like_c(s: &[u8]) -> i32 {
    // Emulate C's atoi behavior closely enough for this program:
    // - skip leading whitespace
    // - optional sign
    // - parse consecutive decimal digits
    // - stop at first non-digit
    // - if no digits, return 0
    // Note: C's atoi has undefined behavior on overflow; we saturate to i32 bounds
    // to avoid panics while keeping deterministic behavior.
    let mut i = 0usize;
    while i < s.len() && s[i].is_ascii_whitespace() {
        i += 1;
    }

    let mut sign: i64 = 1;
    if i < s.len() {
        if s[i] == b'+' {
            i += 1;
        } else if s[i] == b'-' {
            sign = -1;
            i += 1;
        }
    }

    let mut val: i64 = 0;
    let mut any = false;
    while i < s.len() {
        let c = s[i];
        if !c.is_ascii_digit() {
            break;
        }
        any = true;
        val = val * 10 + (c - b'0') as i64;
        i += 1;
    }

    if !any {
        return 0;
    }

    val *= sign;
    if val > i32::MAX as i64 {
        i32::MAX
    } else if val < i32::MIN as i64 {
        i32::MIN
    } else {
        val as i32
    }
}

fn func(s: &[u8], n: usize, m: i32) -> i32 {
    // C version temporarily writes NULs into the string to split it.
    // Here we compute the same splits by slicing.
    let mut sum: i32 = 0;
    let mut start: usize = 0;

    for i in 0..n {
        if (m & (1i32 << i)) != 0 {
            // Split after position i (i.e., between s[i] and s[i+1]).
            let end = i + 1;
            sum = sum.wrapping_add(atoi_like_c(&s[start..end]));
            start = end;
        }
    }
    sum = sum.wrapping_add(atoi_like_c(&s[start..n]));
    sum
}

fn main() {
    // Read all input; emulate scanf("%d%s") tokenization by whitespace.
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
        let paper = match it.next() {
            Some(p) => p.as_bytes().to_vec(),
            None => break,
        };

        if target == 0 {
            break;
        }

        let n = paper.len();
        let mut res: i32 = 0;
        let mut best: i32 = -1;
        let mut cnt: i32 = 0;

        // for(i=(1<<(n-1)); i<(1<<n); ++i)
        // Note: In C, shifting by >= width is undefined; here we assume inputs keep n reasonable.
        let start = 1i32 << ((n as i32) - 1);
        let end = 1i32 << (n as i32);

        let mut i = start;
        while i < end {
            let t = func(&paper, n, i);
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
                out.push(paper[idx] as char);
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
