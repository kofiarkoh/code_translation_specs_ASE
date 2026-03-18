use std::io::{self, Read};

const MAX: usize = 1_000_002;

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    // C-like integer parser (supports leading '-'; assumes valid input as in C code)
    fn in_i32(bytes: &[u8], idx: &mut usize) -> i32 {
        // Skip any non-digit/non-'-' (C code doesn't, but AOJ inputs are well-formed;
        // still, this makes it robust without changing intended behavior on valid inputs).
        while *idx < bytes.len() && bytes[*idx].is_ascii_whitespace() {
            *idx += 1;
        }
        if *idx >= bytes.len() {
            return 0;
        }
        let mut c = bytes[*idx];
        *idx += 1;

        let mut n: i32 = 0;
        if c == b'-' {
            if *idx < bytes.len() {
                c = bytes[*idx];
                *idx += 1;
            } else {
                return 0;
            }
            while c >= b'0' {
                // (c & 0xf) matches C code
                n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
                if *idx >= bytes.len() {
                    break;
                }
                c = bytes[*idx];
                *idx += 1;
            }
            return -n;
        }

        while c >= b'0' {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
            if *idx >= bytes.len() {
                break;
            }
            c = bytes[*idx];
            *idx += 1;
        }
        n
    }

    let n = in_i32(bytes, &mut idx) as usize;

    // Arrays (global in C). Use Vec with fixed MAX to preserve indexing behavior.
    let mut m = vec![0i32; MAX];
    let mut hi = vec![0usize; MAX];
    let mut a = vec![0usize; MAX];
    let mut s = vec![0i64; MAX];
    let mut dp = vec![0i64; MAX];
    let mut btmp = vec![0i64; MAX]; // temporary buffer used in calc

    // memo holds parent indices (or -1)
    let mut memo = vec![0i32; n];

    // First pass: read nodes, count roots and child counts
    let mut bsz: usize = 0;
    for i in 0..n {
        m[i] = in_i32(bytes, &mut idx);
        let l = in_i32(bytes, &mut idx);
        memo[i] = l;
        if l == -1 {
            bsz += 1;
        } else {
            let p = l as usize;
            hi[p] += 1;
        }
    }

    // base: roots
    let mut base = vec![0usize; bsz];

    // to: adjacency lists, allocated to exact sizes like C malloc(hi[i]<<2)
    let mut to: Vec<Vec<usize>> = (0..n).map(|i| Vec::with_capacity(hi[i])).collect();

    // Reset hi to 0 (C: memset(hi, 0, N<<2))
    for i in 0..n {
        hi[i] = 0;
    }

    // Second pass: fill base and adjacency lists
    bsz = 0;
    for i in 0..n {
        let l = memo[i];
        if l == -1 {
            base[bsz] = i;
            bsz += 1;
        } else {
            let p = l as usize;
            to[p].push(i);
            hi[p] += 1;
        }
    }

    // calc(u): compute s[u], dp[u] based on children
    let mut calc = |u: usize,
                    m: &Vec<i32>,
                    hi: &Vec<usize>,
                    to: &Vec<Vec<usize>>,
                    s: &mut Vec<i64>,
                    dp: &mut Vec<i64>,
                    btmp: &mut Vec<i64>| {
        let mut sz: usize = 0;
        s[u] = m[u] as i64;
        dp[u] = 0;

        for &v in to[u].iter().take(hi[u]) {
            btmp[sz] = s[v];
            sz += 1;
            s[u] += s[v];
            dp[u] += dp[v];
        }

        // C qsort with comparator that sorts descending (returns 1 when a<b).
        // So we sort descending.
        let slice = &mut btmp[..sz];
        slice.sort_by(|x, y| y.cmp(x));

        for (i, &val) in slice.iter().enumerate() {
            dp[u] += val * (i as i64);
        }
    };

    // Iterative postorder traversal using explicit stack `que` and per-node child index `a`
    let mut que = vec![0usize; MAX];
    let mut ans: i64 = 0;

    for r in base.iter().take(bsz) {
        que[1] = *r;
        let mut top: usize = 1;

        while top != 0 {
            let t = que[top];
            if a[t] < hi[t] {
                let child = to[t][a[t]];
                a[t] += 1;
                top += 1;
                que[top] = child;
            } else {
                calc(t, &m, &hi, &to, &mut s, &mut dp, &mut btmp);
                top -= 1;
            }
        }
        ans += dp[*r];
    }

    print!("{}\n", ans);
}
