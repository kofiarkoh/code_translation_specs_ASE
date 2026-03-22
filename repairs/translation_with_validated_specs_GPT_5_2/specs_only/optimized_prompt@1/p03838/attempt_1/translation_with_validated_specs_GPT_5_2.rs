use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior for invalid/missing input is unspecified by the spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // We need the minimum number of operations to transform a -> b using:
    // 1) x := x + 1
    // 2) x := -x
    //
    // Any optimal sequence uses at most one sign flip:
    // - Two flips cancel (net effect identity) and only add cost.
    // - With only +1 available, after a flip you can only increase.
    //
    // Therefore, the minimum is:
    // - If b >= a: direct increments: b - a
    // - Else (b < a): either
    //   * flip first: a -> -a (1 op), then increment to b if b >= -a: (b - (-a))
    //   * or increment to -b (if -b >= a), then flip: ( -b - a ) + 1
    //
    // Use i128 for intermediate arithmetic to avoid overflow in cases like -i64::MIN.
    let a128 = a as i128;
    let b128 = b as i128;

    let ans: i128 = if b128 >= a128 {
        b128 - a128
    } else {
        let mut best: Option<i128> = None;

        // Option 1: flip first, then increment (only possible if b >= -a).
        let neg_a = -a128;
        if b128 >= neg_a {
            let cost = 1 + (b128 - neg_a);
            best = Some(cost);
        }

        // Option 2: increment first to reach -b, then flip (only possible if -b >= a).
        let neg_b = -b128;
        if neg_b >= a128 {
            let cost = (neg_b - a128) + 1;
            best = Some(match best {
                Some(x) => x.min(cost),
                None => cost,
            });
        }

        // For b < a, at least one of the above must be feasible.
        best.unwrap()
    };

    // Spec guarantees the required operation count fits in signed 64-bit.
    println!("{}", ans as i64);
}
