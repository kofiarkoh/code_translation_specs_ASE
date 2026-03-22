use std::io::{self, Read};

const MOD: i64 = 998_244_353;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    // Preserve the specified Euclidean algorithm and sign behavior:
    // - gcd(a, 0) = a
    // - gcd(a, b) = gcd(b, a % b) with Rust's % semantics on negatives (like C).
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(s) => s.parse::<i64>().unwrap(),
        None => return,
    };

    // Preconditions assume n >= 0 and allocation succeeds.
    let nn = n as usize;
    let mut a: Vec<i64> = Vec::with_capacity(nn);
    for _ in 0..nn {
        let v = it.next().unwrap().parse::<i64>().unwrap();
        a.push(v);
    }

    // Sort in nondecreasing order.
    // The spec notes potential overflow in a subtraction-based comparator; we avoid inventing that
    // exact UB and instead implement the required nondecreasing order directly.
    a.sort();

    let mut ans: i64 = 1;
    for (i, &val) in a.iter().enumerate() {
        let g = gcd(val, i as i64);
        // Spec: no guarantees if overflow occurs in multiplication before modulo.
        // Emulate 64-bit signed wraparound multiplication (common two's-complement behavior).
        ans = ans.wrapping_mul(g).wrapping_rem(MOD);
    }

    println!("{}", ans);
}
