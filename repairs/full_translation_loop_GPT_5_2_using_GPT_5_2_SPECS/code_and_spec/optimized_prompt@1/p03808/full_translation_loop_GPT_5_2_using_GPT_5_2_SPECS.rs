use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // C code assumes N > 0; spec notes N=0 is not well-defined.
    // We keep behavior aligned with the intended domain (N positive).
    let n_usize = n as usize;

    // A has size N+1, with A[N] = A[0] (cyclic).
    let mut a: Vec<i64> = vec![0; n_usize + 1];
    for i in 0..n_usize {
        a[i] = it.next().unwrap().parse().unwrap();
    }
    if n_usize > 0 {
        a[n_usize] = a[0];
    }

    // diff has indices 0..=N (we use 0..=n_usize).
    let mut diff: Vec<i64> = vec![0; n_usize + 1];

    let mut sum: i64 = 0;
    // In C: for (i=1; i<=N; i++) { sum += A[i]; diff[i] = A[i]-A[i-1]; }
    for i in 1..=n_usize {
        sum += a[i];
        diff[i] = a[i] - a[i - 1];
    }

    let denom: i64 = n * (n + 1) / 2;
    if sum % denom != 0 {
        print!("NO\n");
        return;
    }

    sum /= denom; // This is S in the spec / C code.

    // In C: for (i=1, diff[0]=0; i<=N; i++) { ... }
    diff[0] = 0;
    let mut i_reached: usize = 1;
    while i_reached <= n_usize {
        // C does integer division without checking divisibility; spec requires divisibility.
        // We follow C semantics (source-of-truth for edge behavior) and keep the same checks.
        diff[i_reached] = (sum - diff[i_reached]) / n;
        if diff[i_reached] < 0 || diff[i_reached] > sum {
            break;
        } else {
            diff[0] += diff[i_reached];
        }
        i_reached += 1;
    }

    if i_reached <= n_usize || diff[0] != sum {
        print!("NO\n");
        return;
    }

    // Final weighted consistency check:
    // C: for (i=1; i<=N; i++) A[N] -= diff[i] * (N - i + 1);
    //    if (A[N] != 0) printf("No\n"); else printf("YES\n");
    for i in 1..=n_usize {
        let weight: i64 = n - (i as i64) + 1;
        a[n_usize] -= diff[i] * weight;
    }

    if a[n_usize] != 0 {
        // Note: C prints "No" (lowercase o) here; spec allows "No" as distinct output.
        print!("No\n");
    } else {
        print!("YES\n");
    }
}
