use std::io::{self, Read};

fn ok(k: i64, r: i64, b: i64) -> bool {
    // Narrowest reasonable interpretation from the spec:
    // There is some monotone feasibility predicate in k, checked via binary search.
    // The only concrete relationship mentioned is "involving r and b" with "resource and budget".
    // We interpret feasibility as requiring k units from each constraint: k <= r and k <= b.
    // This yields a monotone predicate suitable for binary search and preserves the "-1" empty case.
    k <= r && k <= b
}

fn main() {
    // Read two long long integers r and b.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let r: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let b: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Preconditions say non-negative, but we do not enforce; we keep behavior defined for any i64.
    // Binary search for maximum k such that ok(k, r, b) holds.
    // Output is "maximum k ... adjusted by subtracting one", and at least -1.
    //
    // We search k in [0, hi], where hi is the maximum possible feasible k under our predicate.
    // Use i128 to avoid overflow when computing hi and mid.
    let hi = if r < b { r } else { b };
    let mut lo: i128 = 0;
    let mut hi: i128 = hi as i128;

    // If hi is negative, no non-negative k can satisfy; result should be -1 after subtracting one.
    if hi < 0 {
        println!("-1");
        return;
    }

    // Standard upper-bound binary search for last true.
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if ok(mid as i64, r, b) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }

    // Adjust by subtracting one; ensure at least -1.
    let ans = (lo as i64).saturating_sub(1);
    println!("{}", ans);
}
