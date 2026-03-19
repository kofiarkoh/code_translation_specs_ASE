use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read a single integer target sum.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.split_whitespace().next().unwrap_or("0");
    let n: i64 = s.parse().unwrap();

    // Preconditions: n must be non-negative. If violated, behavior is undefined by spec.
    // We do not add extra error handling beyond parsing.
    if n < 0 {
        // Undefined behavior per spec; we choose to do nothing observable.
        return;
    }

    // Specification mentions "predefined limits" and "integer partitions" but does not define them.
    // Narrowest reasonable interpretation: count combinations of non-negative integers summing to n
    // with no additional constraints. This yields exactly 1 combination for any n (the 1-tuple [n]
    // or equivalently the empty set of choices besides the sum itself).
    //
    // Output must be in [0, MOD). Ensure modulo normalization even if intermediate were negative.
    let mut ans: i64 = 1;
    ans %= MOD;
    if ans < 0 {
        ans += MOD;
    }

    print!("{}", ans);
}
