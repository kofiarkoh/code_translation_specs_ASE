use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as integers (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: i32 = it.next().unwrap().parse().unwrap();

    // Fixed internal capacity is 1000; precondition says n <= 1000.
    // Use 1-based indexing to match the C code's behavior exactly.
    let mut a: Vec<i32> = vec![0; n + 1];
    for i in 1..=n {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    let mut k: i32 = 1;
    while k <= m {
        // One pass over adjacent pairs; swap if left remainder > right remainder.
        // Note: In C, the loop condition uses the comma operator:
        //   for(i=1,j=2; i<n, j<=n; i++,j++)
        // which effectively means only `j<=n` controls the loop.
        // With i starting at 1 and j at 2 and incrementing together, this is
        // equivalent to iterating j=2..=n (i=j-1), i.e., adjacent pairs.
        for j in 2..=n {
            let i = j - 1;
            if a[i] % k > a[j] % k {
                a.swap(i, j);
            }
        }
        k += 1;
    }

    let mut out = String::new();
    for i in 1..=n {
        out.push_str(&format!("{}\n", a[i]));
    }
    print!("{out}");
}
