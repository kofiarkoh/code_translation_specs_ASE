use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i64 (matches C's long long on typical platforms).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Use 3-length arrays to preserve the 1-based indexing used in the C code.
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
    // we use wrapping arithmetic throughout the computations.
    let left = a[1]
        .wrapping_mul(t[1])
        .wrapping_add(a[2].wrapping_mul(t[2]));
    let right = b[1]
        .wrapping_mul(t[1])
        .wrapping_add(b[2].wrapping_mul(t[2]));

    if left == right {
        print!("infinity\n");
        return;
    }

    if a[1] > b[1] {
        // Swap a[1] <-> b[1] and a[2] <-> b[2] using the same add/sub trick as C,
        // with wrapping to mimic typical machine behavior on overflow.
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
        // C's / and % on signed integers truncate toward zero; Rust matches that.
        if x % denom == 0 {
            let ans = x / denom;
            print!("{}\n", ans.wrapping_mul(2));
        } else {
            let ans = x / denom;
            print!("{}\n", ans.wrapping_mul(2).wrapping_add(1));
        }
    }
}
