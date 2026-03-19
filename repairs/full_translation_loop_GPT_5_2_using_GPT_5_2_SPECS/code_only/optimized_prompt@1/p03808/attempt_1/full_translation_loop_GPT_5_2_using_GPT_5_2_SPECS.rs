use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i64 (matches C long long on typical platforms).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // Arrays sized to match C: A[100001], diff[100001]
    // C writes A[N] = A[0], and uses indices 0..=N, so N must be <= 100000.
    // We keep the same fixed capacity; if N is out of range, C would have UB.
    let mut a = vec![0i64; 100001];
    for i in 0..n {
        if let Some(v) = it.next() {
            a[i as usize] = v.parse().unwrap();
        } else {
            a[i as usize] = 0; // if input is short, mimic "whatever" minimally; C would be undefined.
        }
    }
    a[n as usize] = a[0];

    let mut sum: i64 = 0;
    let mut diff = vec![0i64; 100001];

    for i in 1..=n {
        sum = sum.wrapping_add(a[i as usize]);
        diff[i as usize] = a[i as usize].wrapping_sub(a[(i - 1) as usize]);
    }

    // denom = N*(N+1)/2 with C signed overflow semantics (UB in C),
    // but we use wrapping to emulate typical two's-complement behavior.
    let denom = n
        .wrapping_mul(n.wrapping_add(1))
        .wrapping_div(2);

    if denom == 0 {
        // In C, sum % 0 is UB. We choose to avoid panic and follow a conservative "NO".
        print!("NO\n");
        return;
    }

    if sum % denom != 0 {
        print!("NO\n");
    } else {
        sum /= denom;

        diff[0] = 0;
        let mut i: i64 = 1;
        while i <= n {
            let idx = i as usize;
            // (sum - diff[i]) / N, using wrapping for subtraction; division by 0 handled below.
            let numer = sum.wrapping_sub(diff[idx]);
            if n == 0 {
                // Would be UB in C; avoid panic.
                break;
            }
            diff[idx] = numer / n;

            if diff[idx] < 0 || diff[idx] > sum {
                break;
            } else {
                diff[0] = diff[0].wrapping_add(diff[idx]);
            }
            i += 1;
        }

        if i <= n || diff[0] != sum {
            print!("NO\n");
        } else {
            // for (i = 1; i <= N; i++) A[N] -= diff[i] * (N - i + 1);
            for i in 1..=n {
                let term = diff[i as usize].wrapping_mul(n.wrapping_sub(i).wrapping_add(1));
                a[n as usize] = a[n as usize].wrapping_sub(term);
            }
            if a[n as usize] != 0 {
                // Note: C prints "No\n" (lowercase o) here.
                print!("No\n");
            } else {
                print!("YES\n");
            }
        }
    }

    // stdout is line-buffered typically; explicit flush not necessary in Rust for normal exit.
}
