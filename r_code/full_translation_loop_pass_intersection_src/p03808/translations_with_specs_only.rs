use std::io::{self, Read};

fn main() {
    // Read entire stdin and parse as signed 64-bit integers (C long long analogue).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let n_usize = n as usize;

    // Fixed-size array behavior in the spec: program uses length 100001 and accesses A[N].
    // We allocate N+1 and explicitly set A[N] = A[0] after reading.
    let mut a: Vec<i64> = vec![0; n_usize + 1];
    for i in 0..n_usize {
        a[i] = it.next().unwrap().parse::<i64>().unwrap();
    }
    a[n_usize] = a[0];

    // sum = Σ_{i=1..N} A[i], which includes A[N] = A[0] once more.
    let mut sum: i64 = 0;
    for i in 1..=n_usize {
        sum = sum.wrapping_add(a[i]);
    }

    // T = N*(N+1)/2 in signed 64-bit; use wrapping to emulate C long long overflow behavior.
    let t: i64 = n
        .wrapping_mul(n.wrapping_add(1))
        .wrapping_div(2);

    // Divisibility check: sum must be divisible by T.
    // (If T == 0 due to overflow, C behavior would be undefined; spec says unspecified.
    // We keep narrow behavior: avoid panic and reject.)
    if t == 0 || sum % t != 0 {
        print!("NO\n");
        return;
    }

    let s: i64 = sum / t;

    // Compute x_i and validate range; also accumulate Σ x_i.
    let mut xs_sum: i64 = 0;
    let mut xs: Vec<i64> = vec![0; n_usize + 1]; // 1..=N used
    for i in 1..=n_usize {
        let d_i: i64 = a[i].wrapping_sub(a[i - 1]);
        let numer: i64 = s.wrapping_sub(d_i);
        // Integer division as in C for signed values: Rust matches truncation toward zero.
        let x_i: i64 = numer / n;
        xs[i] = x_i;

        if x_i < 0 || x_i > s {
            print!("NO\n");
            return;
        }
        xs_sum = xs_sum.wrapping_add(x_i);
    }

    if xs_sum != s {
        print!("NO\n");
        return;
    }

    // Final weighted check:
    // start from A[N] (== A[0]) and subtract x_i * (N - i + 1) for i=1..N.
    let mut chk: i64 = a[n_usize];
    for i in 1..=n_usize {
        let weight: i64 = n.wrapping_sub(i as i64).wrapping_add(1);
        let term: i64 = xs[i].wrapping_mul(weight);
        chk = chk.wrapping_sub(term);
    }

    if chk != 0 {
        // Specific late failing condition prints "No" (case-sensitive).
        print!("No\n");
        return;
    }

    print!("YES\n");
}
