use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut n: i64 = it.next().unwrap().parse().unwrap();
    let mut a: i32 = it.next().unwrap().parse().unwrap();
    let mut b: i32 = it.next().unwrap().parse().unwrap();

    // C: N %= 12; (for signed, remainder keeps sign of dividend)
    n %= 12;

    // C loop: i is int, starts at 0, while (i < N) with mixed signed compare.
    // In C, i is promoted to long long for comparison. If N is negative, loop never runs.
    let mut i: i64 = 0;
    while i < n {
        // C int arithmetic can overflow (UB). Use wrapping to emulate typical 2's complement behavior.
        a = a.wrapping_sub(b);
        i += 1;
        if i == n {
            break;
        }
        b = a.wrapping_add(b);
        i += 1;
    }

    // Match C printf("%d %d\n", A, B);
    print!("{} {}\n", a, b);
}
