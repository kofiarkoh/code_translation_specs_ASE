use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i64 (matches C's long long on typical platforms).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut t = [0i64; 3];
    let mut a = [0i64; 3];
    let mut b = [0i64; 3];

    t[1] = it.next().unwrap().parse::<i64>().unwrap();
    t[2] = it.next().unwrap().parse::<i64>().unwrap();
    a[1] = it.next().unwrap().parse::<i64>().unwrap();
    a[2] = it.next().unwrap().parse::<i64>().unwrap();
    b[1] = it.next().unwrap().parse::<i64>().unwrap();
    b[2] = it.next().unwrap().parse::<i64>().unwrap();

    // In C, signed overflow is UB. To emulate typical two's-complement wraparound behavior,
    // use wrapping arithmetic throughout the computations that may overflow.
    let left = a[1].wrapping_mul(t[1]).wrapping_add(a[2].wrapping_mul(t[2]));
    let right = b[1].wrapping_mul(t[1]).wrapping_add(b[2].wrapping_mul(t[2]));
    if left == right {
        print!("infinity\n");
        return;
    }

    if a[1] > b[1] {
        // Preserve the C swap-by-add/sub semantics with wrapping.
        a[1] = a[1].wrapping_add(b[1]);
        b[1] = a[1].wrapping_sub(b[1]);
        a[1] = a[1].wrapping_sub(b[1]);

        a[2] = a[2].wrapping_add(b[2]);
        b[2] = a[2].wrapping_sub(b[2]);
        a[2] = a[2].wrapping_sub(b[2]);
    }

    let x = b[1].wrapping_sub(a[1]).wrapping_mul(t[1]);
    let y = a[2].wrapping_sub(b[2]).wrapping_mul(t[2]);

    if x > y {
        print!("0\n");
    } else {
        let denom = y.wrapping_sub(x);
        // C would have undefined behavior on division/mod by zero; we mirror that by
        // letting Rust panic if denom == 0 (should be unreachable given earlier check).
        let rem = x % denom;
        let q = x / denom;
        if rem == 0 {
            let ans = q.wrapping_mul(2);
            print!("{}\n", ans);
        } else {
            let ans = q.wrapping_mul(2).wrapping_add(1);
            print!("{}\n", ans);
        }
    }
}
