use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // We use 1-based indexing to mirror the C code and the spec's A1..A_{N+1}, B1..B_N.
    // Allocate a bit extra so B[n+1] exists (as in the C code).
    let mut a = vec![0i64; n + 2]; // indices: 0..=n+1
    let mut b = vec![0i64; n + 2]; // indices: 0..=n+1

    for i in 1..=n + 1 {
        a[i] = it.next().unwrap().parse().unwrap();
    }
    for i in 1..=n {
        b[i] = it.next().unwrap().parse().unwrap();
    }

    // Sentinels as in C (B[0]=0, B[N+1]=0)
    b[0] = 0;
    b[n + 1] = 0;

    let mut s: i64 = 0;

    // Process demands in order, consuming from current then next availability.
    // This loop structure matches the C implementation, which effectively carries
    // remaining demand forward via b[i] after consuming from a[i].
    for i in 1..=n + 1 {
        // First, satisfy previous remaining demand (b[i-1]) from current availability a[i].
        let take_prev = if a[i] > b[i - 1] { b[i - 1] } else { a[i] };
        s += take_prev;
        a[i] = if a[i] > b[i - 1] { a[i] - b[i - 1] } else { 0 };

        // Then, satisfy current demand b[i] from remaining a[i].
        let take_cur = if a[i] > b[i] { b[i] } else { a[i] };
        s += take_cur;

        // Update remaining demand b[i] after consuming from a[i].
        b[i] = if a[i] > b[i] { 0 } else { b[i] - a[i] };
    }

    print!("{s}\n");
}
