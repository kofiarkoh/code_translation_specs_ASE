use std::io::{self, Read};

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: u64 = it.next().unwrap().parse().unwrap();
    let b: u64 = it.next().unwrap().parse().unwrap();
    let k: u64 = it.next().unwrap().parse().unwrap();

    let g = gcd(a, b);

    // The intended behavior (per failing sample) is to list common divisors in descending order
    // and output the k-th one (1-indexed). This is equivalent to:
    // - enumerate all divisors of gcd(a,b)
    // - sort descending
    // - pick k-th
    //
    // We keep linear iteration as allowed by the spec's non-functional constraint.
    let mut divisors: Vec<u64> = Vec::new();
    for d in 1..=g {
        if g % d == 0 {
            divisors.push(d);
        }
    }
    divisors.sort_unstable_by(|x, y| y.cmp(x)); // descending

    // If k is out of range, output is undefined; narrowest interpretation: output nothing.
    if k >= 1 && (k as usize) <= divisors.len() {
        print!("{}", divisors[(k - 1) as usize]);
    }
}
