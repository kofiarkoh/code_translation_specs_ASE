use std::io::{self, Read};

fn num_digits(mut x: u64) -> u32 {
    // Decimal digit count for non-negative integers; 0 has 1 digit.
    if x == 0 {
        return 1;
    }
    let mut d: u32 = 0;
    while x > 0 {
        d += 1;
        x /= 10;
    }
    d
}

fn pow10_u64(exp: u32) -> u64 {
    // Preconditions state no overflow occurs in intermediate computations.
    let mut p: u64 = 1;
    for _ in 0..exp {
        p *= 10;
    }
    p
}

fn main() {
    // No explicit error handling per specification; parsing failures will panic.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<u64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: u64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    // Track three smallest concatenation values with multiplicity.
    let mut first: u64 = u64::MAX;
    let mut second: u64 = u64::MAX;
    let mut third: u64 = u64::MAX;

    // Quadratic enumeration of ordered pairs of distinct elements.
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let x = a[i];
            let y = a[j];
            let d = num_digits(y);
            let concat = x * pow10_u64(d) + y;

            if concat <= first {
                third = second;
                second = first;
                first = concat;
            } else if concat <= second {
                third = second;
                second = concat;
            } else if concat <= third {
                third = concat;
            }
        }
    }

    // Preconditions guarantee at least three ordered distinct pairs exist.
    print!("{}", third);
}
