use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i64 (C long long).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let n_usize = n as usize;

    // Arrays sized like the C code (100001), and we will index up to N.
    let mut a = vec![0i64; 100001];
    for i in 0..n_usize {
        a[i] = it.next().unwrap().parse::<i64>().unwrap();
    }
    a[n_usize] = a[0];

    let mut sum: i64 = 0;
    let mut diff = vec![0i64; 100001];

    // C loop: for (i = 1; i <= N; i++)
    for i in 1..=n_usize {
        // Use wrapping to emulate C signed overflow behavior as closely as possible.
        sum = sum.wrapping_add(a[i]);
        diff[i] = a[i].wrapping_sub(a[i - 1]);
    }

    // denom = N*(N+1)/2 with wrapping like C.
    let denom = n
        .wrapping_mul(n.wrapping_add(1))
        .wrapping_div(2);

    if denom == 0 || sum % denom != 0 {
        print!("NO\n");
        io::stdout().flush().ok();
        return;
    }

    sum = sum.wrapping_div(denom);

    diff[0] = 0;
    let mut i_idx: usize = 1;
    while i_idx <= n_usize {
        // C: diff[i] = (sum - diff[i]) / N;
        let val = sum.wrapping_sub(diff[i_idx]).wrapping_div(n);
        diff[i_idx] = val;

        if diff[i_idx] < 0 || diff[i_idx] > sum {
            break;
        } else {
            diff[0] = diff[0].wrapping_add(diff[i_idx]);
        }
        i_idx += 1;
    }

    if i_idx <= n_usize || diff[0] != sum {
        print!("NO\n");
        io::stdout().flush().ok();
        return;
    }

    // C: for (i = 1; i <= N; i++) A[N] -= diff[i] * (N - i + 1);
    for i in 1..=n_usize {
        let factor = n.wrapping_sub(i as i64).wrapping_add(1);
        let sub = diff[i].wrapping_mul(factor);
        a[n_usize] = a[n_usize].wrapping_sub(sub);
    }

    if a[n_usize] != 0 {
        // Note: C prints "No\n" (lowercase o) here; preserve exactly.
        print!("No\n");
    } else {
        print!("YES\n");
    }

    io::stdout().flush().ok();
}

// Needed for flush()
use std::io::Write;
