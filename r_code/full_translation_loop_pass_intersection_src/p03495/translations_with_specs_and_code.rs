use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Per spec, n is non-negative and values are in 1..=n.
    // We store frequencies for indices 1..=n (index 0 unused).
    let n_usize = n as usize;
    let mut c = vec![0_i64; n_usize + 1];

    for _ in 0..n_usize {
        let a: usize = it.next().unwrap().parse().unwrap();
        c[a] += 1;
    }

    // Sort the multiset of frequencies for values 1..=n in nondecreasing order.
    // (Index 0 is unused; we sort only c[1..=n].)
    c[1..=n_usize].sort_unstable();

    // Sum the first (n - k) counts after sorting, excluding zeros.
    // Note: The C code iterates i=1..=N-K over the sorted array c[1..=N],
    // which corresponds to taking the smallest (n-k) frequencies.
    let take = (n - k) as usize;
    let mut ans: i64 = 0;

    for &freq in c[1..=n_usize].iter().take(take) {
        if freq > 0 {
            ans += freq;
        }
    }

    print!("{ans}\n");
}
