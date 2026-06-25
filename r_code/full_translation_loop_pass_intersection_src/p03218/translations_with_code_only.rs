use std::cmp::Ordering;
use std::io::{self, Read};

const P: i64 = 998_244_353;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    // Read entire stdin, parse as i64 like scanf("%lld", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    // C code does: malloc(sizeof(int) * N) with int = long long.
    // If N is negative, behavior is undefined in C; here we emulate a likely failure by
    // producing an empty vector (and the loop won't run), which is a reasonable fallback.
    let n_usize = if n > 0 { n as usize } else { 0usize };

    let mut a: Vec<i64> = Vec::with_capacity(n_usize);
    for _ in 0..n_usize {
        if let Some(x) = it.next() {
            a.push(x.parse().unwrap());
        } else {
            // If input is short, C's scanf would leave elements uninitialized (UB later).
            // We choose 0 to keep execution defined in Rust.
            a.push(0);
        }
    }

    // qsort with comparator:
    // sub = *(int*)a - *(int*)b; then compare sub to 0.
    // That subtraction can overflow in C (UB). We emulate typical two's-complement wrap
    // using wrapping_sub, then compare the wrapped result to 0.
    a.sort_by(|&x, &y| {
        let sub = x.wrapping_sub(y);
        if sub < 0 {
            Ordering::Less
        } else if sub == 0 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    let mut ans: i64 = 1;
    for (i, &val) in a.iter().enumerate() {
        let g = gcd(val, i as i64);
        // C does (ans * g) % p with signed long long; multiplication can overflow (UB).
        // Emulate wraparound multiplication.
        ans = ans.wrapping_mul(g) % P;
    }

    // Match printf("%lld\n", ans);
    print!("{ans}\n");
}
