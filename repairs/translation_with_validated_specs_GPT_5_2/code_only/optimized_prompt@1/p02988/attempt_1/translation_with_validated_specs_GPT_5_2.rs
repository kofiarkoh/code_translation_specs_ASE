use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf would consume tokens separated by whitespace.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // In the C code, failing scanf would leave variables uninitialized (UB),
    // but we assume valid input as typical for such problems.
    let n: i32 = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    // C uses int num[1000] and indexes from 1..=n (1-based).
    // We'll allocate 1000 elements and keep index 0 unused to preserve semantics.
    let mut num = vec![0i32; 1000];

    // for (i = 1; i <= n; i++) scanf("%d", &num[i]);
    // If n is out of bounds, C would have undefined behavior; we avoid panics by
    // only writing when the index is within the allocated array.
    for i in 1..=n {
        if let Some(tok) = it.next() {
            if let Ok(v) = tok.parse::<i32>() {
                let idx = i as isize;
                if (0..(num.len() as isize)).contains(&idx) {
                    num[idx as usize] = v;
                }
            }
        }
    }

    let mut count: i32 = 0;

    // for (i = 1; i < n-1; i++)
    // Use i32 loop to match C int comparisons; guard indices to avoid panics.
    let mut i: i32 = 1;
    while i < n - 1 {
        let i_us = i as isize;
        let i1_us = (i + 1) as isize;
        let i2_us = (i + 2) as isize;

        if (0..(num.len() as isize)).contains(&i_us)
            && (0..(num.len() as isize)).contains(&i1_us)
            && (0..(num.len() as isize)).contains(&i2_us)
        {
            let a = num[i_us as usize];
            let b = num[i1_us as usize];
            let c = num[i2_us as usize];

            if a < b {
                if b < c {
                    count += 1;
                }
            } else if a > b {
                if b > c {
                    count += 1;
                }
            }
        }

        i += 1;
    }

    // Match C's printf("%d\n", count);
    print!("{}\n", count);
}
