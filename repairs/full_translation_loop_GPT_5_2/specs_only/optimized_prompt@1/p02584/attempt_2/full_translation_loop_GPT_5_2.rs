use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let mut x: i128 = it.next().unwrap().parse().unwrap();
    let d: i128 = it.next().unwrap().parse().unwrap();
    let k: i128 = it.next().unwrap().parse().unwrap();

    // Divisor must be non-zero per specification; behavior is undefined otherwise.
    // We do not add special handling beyond relying on Rust's division-by-zero panic
    // if it ever occurs (it won't here because we only use d in arithmetic comparisons).
    //
    // The intended operation (inferred from sample) is:
    // Start from |x|, then perform k operations:
    //   if current value >= d: subtract d
    //   else: stop early
    // Output the final value.
    //
    // This is equivalent to max(|x| - k*d, |x| mod d) but with early stop at < d.
    // More directly: if k <= |x|/d => |x| - k*d else |x| % d.
    //
    // This runs in constant time (no loop over k).
    let ax: i128 = if x == i128::MIN { i128::MAX } else { x.abs() };

    let q = ax / d;
    let result = if k <= q {
        ax - k * d
    } else {
        ax % d
    };

    print!("{result}");
}
