use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let base: i64 = it.next().unwrap().parse().unwrap();
    let target: i64 = it.next().unwrap().parse().unwrap();

    // C long long arithmetic is assumed not to overflow per spec; in Rust we use wrapping
    // to avoid panics in debug builds and to keep behavior defined.
    let n: i64 = base.wrapping_mul(7);

    fn pred(i: i64, target: i64) -> bool {
        // Match C's % semantics for negative i: remainder has the sign of the dividend.
        let r7 = i % 7;
        if r7 == 0 || r7 == 1 {
            return false;
        }
        let v = (9i64.wrapping_mul(i)) % 11;
        v == target
    }

    // Count in 1..=limit (inclusive) using a simple loop.
    fn count_upto(limit: i64, target: i64) -> i64 {
        let mut c: i64 = 0;
        let mut i: i64 = 1;
        while i <= limit {
            if pred(i, target) {
                c = c.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        c
    }

    // Periodic counting with period 693:
    // A = floor(N/693) * count(1..=693) + count(1..=N%693)
    let period: i64 = 693;
    let full_blocks: i64 = n / period;
    let rem: i64 = n % period;

    let per_block = count_upto(period, target);
    let mut a: i64 = full_blocks.wrapping_mul(per_block).wrapping_add(count_upto(rem, target));

    // Subtract window 1..=7
    let mut i: i64 = 1;
    while i <= 7 {
        if pred(i, target) {
            a = a.wrapping_sub(1);
        }
        i = i.wrapping_add(1);
    }

    // Subtract window N-6..=N (inclusive), applied independently even if overlapping.
    let start = n.wrapping_sub(6);
    let mut j: i64 = start;
    while j <= n {
        if pred(j, target) {
            a = a.wrapping_sub(1);
        }
        j = j.wrapping_add(1);
    }

    println!("{}", a);
}
