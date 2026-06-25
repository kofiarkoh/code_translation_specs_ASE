use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as ints like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut a = vec![0i32; n];
    let mut b = vec![0i32; n];
    let mut c = vec![0i32; n];

    for i in 0..n {
        a[i] = it.next().unwrap().parse().unwrap();
        b[i] = it.next().unwrap().parse().unwrap();
        c[i] = it.next().unwrap().parse().unwrap();
    }

    let states: usize = 1usize << n;

    // d[20][1<<16] and fr[20][1<<16] in C; allocate based on n.
    let mut d = vec![vec![1e18f64; states]; n];
    let mut fr = vec![vec![-1i32; states]; n];

    for i in 0..n {
        d[i][1usize << i] = 0.0;
    }

    // f(i,j,k) as in C
    let f = |i: usize, jmask: usize, k: usize, n: usize, b: &Vec<i32>, c: &Vec<i32>| -> f64 {
        if jmask == 0 {
            return 0.0;
        }
        let mut w: i32 = 0;
        for l in 0..n {
            if (jmask & (1usize << l)) != 0 {
                // In C: w += c[l]*20; (int arithmetic)
                w += c[l] * 20;
            }
        }
        // In C: abs(b[k]-b[i]) uses int abs; emulate with i32 and i64 to avoid Rust overflow panic.
        let diff = (b[k] as i64) - (b[i] as i64);
        let adiff = diff.abs() as f64;
        (adiff / 2000.0) * ((w + 70) as f64)
    };

    for jmask in 0..states {
        for i in 0..n {
            // C bug/precedence: if((1<<i)&j==0)continue;
            // In C, this parses as: if ( (1<<i) & (j==0) ) continue;
            // where (j==0) is 0 or 1. We must preserve that exact behavior.
            let cond_bit = (1usize << i) & if jmask == 0 { 1 } else { 0 };
            if cond_bit != 0 {
                continue;
            }

            for k in 0..n {
                if (jmask & (1usize << k)) != 0 {
                    continue;
                }
                let nj = jmask + (1usize << k);
                let cand = d[i][jmask] + f(i, jmask, k, n, &b, &c);
                if d[k][nj] > cand {
                    d[k][nj] = cand;
                    fr[k][nj] = i as i32;
                }
            }
        }
    }

    // In C: k=j-1; with j after loop equals (1<<n). So k = (1<<n)-1.
    let mut k_mask: usize = states - 1;

    // Find j minimizing d[j][k]
    let mut jbest: usize = 0;
    for i in 0..n {
        if d[i][k_mask] < d[jbest][k_mask] {
            jbest = i;
        }
    }

    let mut r = vec![0usize; n];
    r[n - 1] = jbest;

    for i in 1..n {
        let prev = r[n - i];
        let p = fr[prev][k_mask];
        // C assumes reachable; keep behavior (would be UB if -1 used as index).
        let p_usize = p as usize;
        r[n - i - 1] = p_usize;
        k_mask -= 1usize << prev;
    }

    // Output: a[r[0]] ... a[r[n-1]] with spaces, newline at end.
    let mut out = String::new();
    for i in 0..n {
        if i + 1 == n {
            out.push_str(&format!("{}\n", a[r[i]]));
        } else {
            out.push_str(&format!("{} ", a[r[i]]));
        }
    }
    print!("{out}");
}
