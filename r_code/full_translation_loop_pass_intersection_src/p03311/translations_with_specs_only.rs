use std::io::{self, Read};

fn main() {
    // Interpreting the intended task (AtCoder p03311 / "B - ..."):
    // Given A[i], choose an integer b to minimize sum_i |(A[i] - (i+1)) - b|.
    // This is minimized by choosing b as the median of X[i] = A[i] - (i+1).
    // Then output the minimal sum of absolute deviations.
    //
    // This matches the sample:
    // A = [1,5,3,7,9]
    // X = [0,3,0,3,4], median=3 => sum=3+0+3+0+1=7.

    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut x: Vec<i64> = Vec::with_capacity(n);

    for i in 0..n {
        let a: i64 = it.next().unwrap().parse().unwrap();
        // Use i64; input is within "long integer" range per spec.
        // i is usize; convert safely for typical constraints.
        let idx = (i as i64) + 1;
        x.push(a - idx);
    }

    x.sort_unstable();
    let b = x[n / 2];

    let mut ans: i64 = 0;
    for &v in &x {
        ans += (v - b).abs();
    }

    println!("{}", ans);
}
