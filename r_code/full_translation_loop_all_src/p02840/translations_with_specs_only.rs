use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::{self, Read};

fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        return b.abs();
    }
    if b == 0 {
        return a.abs();
    }
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn main() {
    // Read N, X, D as signed 64-bit integers (C++ long long equivalent).
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let x: i64 = it.next().unwrap().parse().unwrap();
    let d: i64 = it.next().unwrap().parse().unwrap();

    // Exceptional behavior per specification.
    if x == 0 && d == 0 {
        println!("1");
        return;
    }
    if d == 0 {
        // Output N + 1
        println!("{}", n + 1);
        return;
    }
    if x == 0 && d != 0 {
        // Output N*(N-1)/2 + 1
        // Use i128 to avoid overflow in intermediate multiplication.
        let nn = n as i128;
        let ans = nn * (nn - 1) / 2 + 1;
        println!("{}", ans as i64);
        return;
    }

    // Normalize by gcd(X, D) as described.
    let g = gcd_i64(x, d);
    let x0 = x / g;
    let d0 = d / g;

    // Hash table: key is remainder class modulo |d0|, value is a list of disjoint merged ranges [l, r].
    // Ranges are over the normalized "base" value:
    // For each k in [0..N], consider sums:
    //   S(k, t) = k*X0 + D0 * t, where t ranges over all sums of chosen indices:
    //   t in [k*(k-1)/2, k*(2N-k-1)/2]
    // Then S(k, t) has fixed remainder (k*X0 mod D0) and spans a contiguous interval in the quotient.
    //
    // This matches the described behavior: iterate over N, update hash table with new ranges, merge as needed.
    let m = d0.abs() as i128; // modulus magnitude for remainder classes
    let mut map: HashMap<i128, Vec<(i128, i128)>> = HashMap::new();

    let nn = n as i128;
    let x0i = x0 as i128;
    let d0i = d0 as i128;

    for k in 0..=nn {
        // Compute t_min and t_max for choosing k elements from {0..N-1}.
        // min sum: 0+1+...+(k-1) = k*(k-1)/2
        // max sum: (N-k)+...+(N-1) = k*(2N-k-1)/2
        let t_min = k * (k - 1) / 2;
        let t_max = k * (2 * nn - k - 1) / 2;

        // Base value in normalized units.
        let base = k * x0i;

        // We want to group by remainder modulo d0 (magnitude).
        // Use Euclidean remainder in [0, m).
        let mut rem = base % m;
        if rem < 0 {
            rem += m;
        }

        // Convert interval of S = base + d0*t into an interval over the quotient q where:
        // S = rem + d0 * q  (with rem fixed per class)
        // Since base ≡ rem (mod d0), (base - rem) is divisible by d0.
        // For t in [t_min, t_max], q ranges over:
        //   q = (base - rem)/d0 + t
        // If d0 is negative, division still exact; use i128 and exactness.
        let offset = (base - rem) / d0i;
        let mut l = offset + t_min;
        let mut r = offset + t_max;
        if l > r {
            std::mem::swap(&mut l, &mut r);
        }

        // Insert and merge into the vector of disjoint ranges for this remainder class.
        let entry = map.entry(rem).or_insert_with(Vec::new);
        entry.push((l, r));
    }

    // Merge ranges per remainder class and count total covered integers.
    let mut total: i128 = 0;
    for (_rem, mut ranges) in map {
        ranges.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut cur_l: i128 = 0;
        let mut cur_r: i128 = -1;
        let mut first = true;

        for (l, r) in ranges {
            if first {
                cur_l = l;
                cur_r = r;
                first = false;
                continue;
            }
            if l <= cur_r + 1 {
                cur_r = max(cur_r, r);
            } else {
                total += cur_r - cur_l + 1;
                cur_l = l;
                cur_r = r;
            }
        }
        if !first {
            total += cur_r - cur_l + 1;
        }
    }

    // Output as long long. The specification states output is non-negative.
    println!("{}", total as i64);
}
