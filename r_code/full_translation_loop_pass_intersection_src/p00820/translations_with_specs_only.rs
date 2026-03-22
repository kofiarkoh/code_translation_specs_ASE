use std::io::{self, Read};

fn count_representations(n: i64) -> i64 {
    // Count tuples (a1..ak), 1<=k<=4, positive integers, nondecreasing, sum of squares = n.
    // Implemented via bounded-depth recursion (depth <= 4) to match the specification.
    fn rec(remaining: i64, min_a: i64, depth_left: i32) -> i64 {
        if remaining <= 0 || depth_left == 0 {
            return 0;
        }

        let mut count = 0i64;

        // a must be >= min_a and a^2 <= remaining
        // Use integer sqrt via floating point as a narrow, practical interpretation;
        // the spec does not define behavior for extremely large n where precision might matter.
        let max_a = (remaining as f64).sqrt().floor() as i64;

        for a in min_a..=max_a {
            let sq = a * a;
            let new_rem = remaining - sq;

            if new_rem == 0 {
                // Found a representation with current term as the last term (k can be 1..4).
                count += 1;
            } else {
                // Continue building a nondecreasing tuple; next term must be >= a.
                count += rec(new_rem, a, depth_left - 1);
            }
        }

        count
    }

    rec(n, 1, 4)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut outputs: Vec<String> = Vec::new();

    for tok in input.split_whitespace() {
        // No explicit error handling per specification; unwrap is acceptable.
        let n: i64 = tok.parse().unwrap();
        if n == 0 {
            break;
        }
        outputs.push(count_representations(n).to_string());
    }

    print!("{}", outputs.join("\n"));
}
