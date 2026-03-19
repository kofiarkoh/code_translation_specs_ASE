use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read entire stdin, split by whitespace, parse as i64.
    // Specification: no explicit error handling; malformed/missing input => undefined.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Spec mentions stack allocation of (length + 1); in Rust, Vec allocates on heap.
    // This preserves functional behavior and determinism; stack-failure UB is not modeled.
    let mut x: Vec<i64> = Vec::with_capacity(n + 1);
    let mut y: Vec<i64> = Vec::with_capacity(m + 1);

    for _ in 0..n {
        x.push(it.next().unwrap().parse().unwrap());
    }
    for _ in 0..m {
        y.push(it.next().unwrap().parse().unwrap());
    }

    // Compute:
    // T = sum_{a<b} (X[b] - X[a]) = sum_i X[i] * (2*i - (n-1)) with i 0-based.
    // K similarly.
    //
    // Spec: signed overflow in intermediate arithmetic is UB in the original;
    // here we use i128 intermediates to avoid overflow and then reduce mod MOD.
    // Also, spec allows negative representatives after modular reductions; we preserve that
    // by using Rust's % (remainder) which keeps the sign of the left operand.
    fn aggregate(seq: &[i64]) -> i64 {
        let len = seq.len() as i128;
        let mut acc: i128 = 0;
        for (i, &v) in seq.iter().enumerate() {
            let i = i as i128;
            let coeff = 2 * i - (len - 1);
            acc += (v as i128) * coeff;
        }
        (acc % (MOD as i128)) as i64
    }

    let t = aggregate(&x);
    let k = aggregate(&y);

    let ans = ((t as i128) * (k as i128) % (MOD as i128)) as i64;
    print!("{ans}\n");
}
