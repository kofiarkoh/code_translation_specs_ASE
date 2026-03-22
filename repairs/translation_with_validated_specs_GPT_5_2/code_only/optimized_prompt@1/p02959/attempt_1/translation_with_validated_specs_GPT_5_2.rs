use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated tokens (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    // C uses arrays sized 200000 and indexes from 1..=N+1, also touches B[0] and B[N+1].
    // Allocate n+2 (0..=n+1) to match those accesses.
    let mut a = vec![0i64; n + 2];
    let mut b = vec![0i64; n + 2];

    for i in 1..=n + 1 {
        a[i] = it.next().unwrap().parse::<i64>().unwrap();
    }
    for i in 1..=n {
        b[i] = it.next().unwrap().parse::<i64>().unwrap();
    }
    b[0] = 0;
    b[n + 1] = 0;

    let mut s: i64 = 0;

    for i in 1..=n + 1 {
        // Emulate the C ternary logic exactly.
        let take1 = if a[i] > b[i - 1] { b[i - 1] } else { a[i] };
        s = s.wrapping_add(take1);

        a[i] = if a[i] > b[i - 1] {
            a[i].wrapping_sub(b[i - 1])
        } else {
            0
        };

        let take2 = if a[i] > b[i] { b[i] } else { a[i] };
        s = s.wrapping_add(take2);

        b[i] = if a[i] > b[i] {
            0
        } else {
            b[i].wrapping_sub(a[i])
        };
    }

    print!("{}\n", s);
}
