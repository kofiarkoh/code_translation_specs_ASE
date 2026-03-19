use std::io::{self, Read};

fn main() {
    // Read entire stdin; no explicit error handling per specification.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let size: i128 = it.next().unwrap().parse().unwrap();
    let q: usize = it.next().unwrap().parse().unwrap();

    let mut out = String::new();

    for _ in 0..q {
        let p: i128 = it.next().unwrap().parse().unwrap();
        let r: i128 = it.next().unwrap().parse().unwrap();

        // d(x) = min(x, size + 1 - x) on a 1-based range.
        // Use i128 to avoid overflow for typical inputs; spec does not define overflow behavior.
        let dp = {
            let other = size + 1 - p;
            if p < other { p } else { other }
        };
        let dr = {
            let other = size + 1 - r;
            if r < other { r } else { other }
        };

        let m = if dp < dr { dp } else { dr };

        // Output ((m - 1) mod 3) + 1, with result in {1,2,3}.
        // For intended inputs m >= 1. If not, behavior is unspecified by the spec.
        let res = ((m - 1).rem_euclid(3)) + 1;

        out.push_str(&res.to_string());
        out.push('\n');
    }

    print!("{out}");
}
