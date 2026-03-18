use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse().unwrap());
    }

    // The sample indicates that the iterative sorting must be applied from divisor m down to 1,
    // using a stable sort each time. This way, the final (last applied) divisor is 1, which
    // doesn't change order, and earlier (larger) divisors determine the final arrangement.
    //
    // This matches the invariant: at the start of each iteration, the order is consistent with
    // criteria established by the previous divisors (i.e., those already applied).
    for d in (1..=m).rev() {
        a.sort_by_key(|&x| x % d);
    }

    let mut out = String::new();
    for x in a {
        out.push_str(&x.to_string());
        out.push('\n');
    }
    print!("{out}");
}
