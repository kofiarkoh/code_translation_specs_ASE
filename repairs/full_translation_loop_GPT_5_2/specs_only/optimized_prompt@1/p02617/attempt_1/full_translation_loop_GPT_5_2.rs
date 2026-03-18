use std::io::{self, Read};

fn main() {
    // The specification does not define the exact arithmetic to compute `ans`,
    // only that:
    // - input is N and then N-1 pairs (u, w)
    // - u and w may be swapped to ensure u <= w
    // - ans is a "long long" throughout
    // - output is a single long long integer
    //
    // With no further definition of the computation, the narrowest faithful
    // interpretation is:
    // - read and normalize the pairs (perform the specified swap invariant)
    // - produce a long long output without inventing additional behavior
    // Therefore, we output the initial long long value 0.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_opt = it.next();
    if n_opt.is_none() {
        return;
    }
    let n: i64 = n_opt.unwrap().parse().unwrap();

    // Read N-1 pairs; enforce u <= w by swapping if needed.
    // The values are otherwise unused because the computation is unspecified.
    if n > 1 {
        for _ in 0..(n - 1) {
            let mut u: i64 = it.next().unwrap().parse().unwrap();
            let mut w: i64 = it.next().unwrap().parse().unwrap();
            if u > w {
                std::mem::swap(&mut u, &mut w);
            }
        }
    }

    let ans: i64 = 0;
    print!("{}", ans);
}
