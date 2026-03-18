use std::io::{self, Read};

fn main() {
    // Read entire stdin, like typical competitive-programming C usage with scanf.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // In the C code, failure to read would leave variables uninitialized (UB),
    // but we assume valid input as typical for such tasks.
    let n: i32 = it.next().unwrap().parse().unwrap();

    // C uses int num[1000] and indexes from 1..=n (1-based), leaving num[0] unused.
    // We'll allocate 1000 elements and keep the same indexing scheme.
    let mut num = vec![0i32; 1000];

    for i in 1..=n {
        let v: i32 = it.next().unwrap().parse().unwrap();
        // If n > 999, the C code would write out of bounds (UB). We mirror the
        // intended behavior for valid inputs; for invalid, Rust would panic.
        num[i as usize] = v;
    }

    let mut count: i32 = 0;

    // C loop: for (i = 1; i < n-1; i++)
    // Note: if n <= 2, the loop doesn't execute.
    for i in 1..(n - 1) {
        let iu = i as usize;
        if num[iu] < num[iu + 1] {
            if num[iu + 1] < num[iu + 2] {
                count += 1;
            }
        } else if num[iu] > num[iu + 1] {
            if num[iu + 1] > num[iu + 2] {
                count += 1;
            }
        }
    }

    print!("{}\n", count);
}
