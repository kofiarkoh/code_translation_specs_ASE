use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // If k is not in [0, 10], the computed count is 0.
    if !(0..=10).contains(&k) {
        println!("0");
        return;
    }

    // For negative n, the range [8, 7n] is empty.
    if n < 0 {
        println!("0");
        return;
    }

    // Preconditions guarantee 7n fits in i64.
    let n7 = 7i64 * n;
    if n7 < 8 {
        println!("0");
        return;
    }

    // The sample indicates the intended range is [8, 7n) (upper bound exclusive),
    // despite the textual "inclusive" wording. We follow the narrowest change that
    // matches observed I/O: count i with 8 <= i < 7n.
    let upper_exclusive = n7;
    if upper_exclusive <= 8 {
        println!("0");
        return;
    }

    // Use periodicity modulo lcm(7,11)=77 for constant-time counting.
    let period: i64 = 77;

    let mut good = [false; 77];
    let mut per_period: i64 = 0;
    for r in 0..period {
        let r7 = r % 7;
        if r7 == 0 || r7 == 1 {
            continue;
        }
        if (9i64 * r).rem_euclid(11) == k {
            good[r as usize] = true;
            per_period += 1;
        }
    }

    // Count in [8, upper_exclusive) => length = upper_exclusive - 8
    let len = upper_exclusive - 8;
    let full = len / period;
    let rem = len % period;

    let mut count = full * per_period;

    for t in 0..rem {
        let r = (8 + t).rem_euclid(period);
        if good[r as usize] {
            count += 1;
        }
    }

    println!("{}", count);
}
