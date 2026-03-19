use std::io::{self, Read};

fn bs(d: &[i32], n: i32, c: i32) -> i32 {
    let mut j: i32 = 0;
    let mut k: i32 = n - 1;
    loop {
        if j <= k {
            // In C: l = (j+k)/2 with int arithmetic.
            // Here j,k are non-negative and small, so i32 is fine.
            let l: i32 = (j + k) / 2;
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
    // Read all input, parse as i32 like scanf("%d").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(x) => x.parse::<i32>().unwrap(),
        None => return,
    };

    // Match fixed-size C arrays.
    let mut a = vec![0i32; 100_000];
    let mut dp = vec![0i32; 100_000];

    for i in 0..n {
        a[i as usize] = it.next().unwrap().parse::<i32>().unwrap();
    }

    // Preserve C behavior; note: if n <= 0, the C code has UB (a[0]).
    // We emulate the same indexing assumptions; for n <= 0 we just avoid panicking.
    if n <= 0 {
        // C would be undefined; choose to print 0 to avoid crash.
        // (No external spec; this is a best-effort to keep program running.)
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
