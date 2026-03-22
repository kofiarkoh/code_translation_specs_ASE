use std::io::{self, Read};

fn bs(d: &[i32], n: i32, c: i32) -> i32 {
    let mut j: i32 = 0;
    let mut k: i32 = n - 1;
    loop {
        if j <= k {
            let l = (j + k) / 2;
            let dl = d[l as usize];
            if dl > c {
                k = l - 1;
            } else if dl < c {
                j = l + 1;
            } else {
                return l;
            }
        } else {
            return k + 1;
        }
    }
}

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // If no input, do nothing (closest to C's behavior under missing input).
    };

    // Match C's fixed-size arrays.
    let mut a = vec![0i32; 100000];
    let mut dp = vec![0i32; 100000];

    for i in 0..n {
        if let Some(x) = it.next() {
            a[i as usize] = x.parse().unwrap();
        } else {
            // Missing input: in C this would leave a[i] uninitialized (UB).
            // We keep default 0; this is a best-effort safe behavior.
            a[i as usize] = 0;
        }
    }

    // The C code assumes n >= 1; if n <= 0, it would access a[0] (UB).
    // We avoid panicking; best-effort: print 0.
    if n <= 0 {
        print!("0\n");
        return;
    }

    dp[0] = -1;
    dp[1] = a[0];
    let mut j: i32 = 1;

    for i in 1..n {
        let ai = a[i as usize];
        if dp[j as usize] < ai {
            dp[(j + 1) as usize] = ai;
            j += 1;
        } else {
            let idx = bs(&dp, j + 1, ai);
            dp[idx as usize] = ai;
        }
    }

    print!("{}\n", j);
}
