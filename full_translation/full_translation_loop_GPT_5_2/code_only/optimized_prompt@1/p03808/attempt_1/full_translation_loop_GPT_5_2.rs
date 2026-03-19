use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // The C code assumes valid input; if missing, behavior is unspecified.
    let n: i64 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    // Arrays sized like the C code: A[100001], diff[100001]
    // Note: C writes A[N] = A[0], and uses indices 0..=N.
    let mut a = vec![0i64; 100001];
    let mut diff = vec![0i64; 100001];

    for i in 0..n {
        a[i as usize] = it.next().unwrap().parse().unwrap();
    }
    a[n as usize] = a[0];

    // Emulate C signed overflow/wraparound on long long using wrapping ops.
    let mut sum: i64 = 0;
    for i in 1..=n {
        let ai = a[i as usize];
        sum = sum.wrapping_add(ai);
        diff[i as usize] = ai.wrapping_sub(a[(i - 1) as usize]);
    }

    // denom = N*(N+1)/2 with C-like wraparound
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
    let mut i: i64 = 1;
    while i <= n {
        // In C: diff[i] = (sum - diff[i]) / N;
        // Division is trunc toward zero in both C and Rust.
        let val = sum.wrapping_sub(diff[i as usize]).wrapping_div(n);
        diff[i as usize] = val;

        if val < 0 || val > sum {
            break;
        } else {
            diff[0] = diff[0].wrapping_add(val);
        }
        i += 1;
    }

    if i <= n || diff[0] != sum {
        print!("NO\n");
        io::stdout().flush().ok();
        return;
    }

    // In C: for (i = 1; i <= N; i++) A[N] -= diff[i] * (N - i + 1);
    for i in 1..=n {
        let factor = n.wrapping_sub(i).wrapping_add(1);
        let sub = diff[i as usize].wrapping_mul(factor);
        let idx_n = n as usize;
        a[idx_n] = a[idx_n].wrapping_sub(sub);
    }

    if a[n as usize] != 0 {
        // Note: C prints "No\n" (lowercase o) here.
        print!("No\n");
    } else {
        print!("YES\n");
    }

    io::stdout().flush().ok();
}

// Needed for flush()
use std::io::Write;
