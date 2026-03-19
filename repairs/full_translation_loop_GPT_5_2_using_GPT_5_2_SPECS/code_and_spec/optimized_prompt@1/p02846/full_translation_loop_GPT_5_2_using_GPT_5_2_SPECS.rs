use std::io::{self, Read};

fn main() {
    // Read all input as i64 values.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace().map(|x| x.parse::<i64>().unwrap());

    let t1 = it.next().unwrap();
    let t2 = it.next().unwrap();
    let mut a1 = it.next().unwrap();
    let mut a2 = it.next().unwrap();
    let mut b1 = it.next().unwrap();
    let mut b2 = it.next().unwrap();

    // Equality check is on original inputs (swapping does not affect it).
    let total_a = a1 * t1 + a2 * t2;
    let total_b = b1 * t1 + b2 * t2;
    if total_a == total_b {
        print!("infinity\n");
        return;
    }

    // Normalize so that a1 <= b1 by swapping the per-time-unit pairs if needed.
    if a1 > b1 {
        std::mem::swap(&mut a1, &mut b1);
        std::mem::swap(&mut a2, &mut b2);
    }

    let x = (b1 - a1) * t1;
    let y = (a2 - b2) * t2;

    if x > y {
        print!("0\n");
    } else {
        // Precondition (from spec): divisor must be non-zero on this path.
        let denom = y - x;
        let q = x / denom;
        if x % denom == 0 {
            print!("{}\n", q * 2);
        } else {
            print!("{}\n", q * 2 + 1);
        }
    }
}
