use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k: i32 = it.next().unwrap().parse().unwrap();

    // 1-based arrays to match C indexing
    let mut a = vec![0i32; n + 2];
    for i in 1..=n {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    let mut b = vec![0i32; n + 3]; // need indices up to N+1
    let mut min_val: i32 = 0;

    // for (k = 1; k <= K && min < N; k++)
    let mut kk: i32 = 1;
    while kk <= k && (min_val as usize) < n {
        min_val = n as i32;

        // for (i = 1; i <= N + 1; i++) B[i] = 0;
        for i in 1..=n + 1 {
            b[i] = 0;
        }

        // Difference array updates
        for i in 1..=n {
            let ai = a[i];

            // if (A[i] >= i) B[1]++; else B[i-A[i]]++;
            if ai >= i as i32 {
                b[1] = b[1].wrapping_add(1);
            } else {
                // In C, i - A[i] is assumed within bounds; keep same arithmetic width.
                let idx = (i as i32 - ai) as usize;
                b[idx] = b[idx].wrapping_add(1);
            }

            // if (A[i] >= N - i) B[N+1]--; else B[i+A[i]+1]--;
            if ai >= (n - i) as i32 {
                b[n + 1] = b[n + 1].wrapping_sub(1);
            } else {
                let idx = (i as i32 + ai + 1) as usize;
                b[idx] = b[idx].wrapping_sub(1);
            }
        }

        // Prefix sum to build next A, track min
        let mut tmp: i32 = 0;
        for i in 1..=n {
            tmp = tmp.wrapping_add(b[i]);
            a[i] = tmp;
            if a[i] < min_val {
                min_val = a[i];
            }
        }

        kk += 1;
    }

    // Output exactly like C: numbers separated by single spaces, trailing newline
    let mut out = String::new();
    if n >= 1 {
        for i in 1..n {
            out.push_str(&format!("{} ", a[i]));
        }
        out.push_str(&format!("{}\n", a[n]));
    }
    print!("{}", out);
    // stdout flush is implicit on program exit; C calls fflush(stdout) but not required here.
}
