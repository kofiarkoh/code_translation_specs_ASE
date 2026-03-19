use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as signed 64-bit integers (like scanf("%lld")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k: i64 = it.next().unwrap().parse().unwrap();

    // C uses a VLA `long long int a[n];` and reads n values, but never uses them.
    // We still consume exactly n integers from input to preserve I/O behavior.
    for _ in 0..n {
        let _x: i64 = it.next().unwrap().parse().unwrap();
    }

    // Compute t with C-like integer arithmetic.
    // Note: In C, if k == 1, (k-1) is 0 and division by zero is UB.
    // Here we let Rust panic similarly (via checked division unwrap) to avoid
    // silently changing behavior.
    let denom = k - 1;
    let t0 = (n - k)
        .checked_div(denom)
        .expect("division by zero (matches C UB for k==1)")
        + 1;

    let mut t = t0;
    // if (k + ((t-1)*(k-1)) < n) t++;
    let lhs = k + (t - 1) * (k - 1);
    if lhs < n {
        t += 1;
    }

    print!("{t}\n");
}
