// AOJ 0648 Art Exhibition
// Translated from C to Rust

use std::io::{self, BufRead};

#[derive(Debug)]
struct Pair {
    a: i64,
    b: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of pairs
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");

    // Preallocate vectors for pairs and cumulative sums
    let mut pairs: Vec<Pair> = Vec::with_capacity(n);
    let mut s: Vec<i64> = vec![0; n + 1];

    // Read pairs of integers
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace();
        let a: i64 = nums.next().unwrap().parse().expect("Invalid input for a");
        let b: i32 = nums.next().unwrap().parse().expect("Invalid input for b");
        pairs.push(Pair { a, b });
    }

    // Sort pairs based on the first element, then the second
    pairs.sort_by(|x, y| {
        if x.a == y.a {
            x.b.cmp(&y.b)
        } else {
            x.a.cmp(&y.a)
        }
    });

    const INF: i64 = 0x1010101010101010;
    let mut min = INF;
    let mut max = i64::MIN;

    // Compute the maximum value based on the specification
    for i in 1..=n {
        let t = s[i - 1] - pairs[i - 1].a; // pairs are 0-indexed in Rust
        if t < min {
            min = t;
        }
        s[i] = s[i - 1] + pairs[i - 1].b;
        let t = s[i] - pairs[i - 1].a - min;
        if t > max {
            max = t;
        }
    }

    // Output the maximum computed value
    println!("{}", max);
}
```

### Notes:
1. **Input Handling**: The Rust code uses `io::stdin` and `BufRead` to read input, which is idiomatic and safe.
2. **Data Structures**: A `Pair` struct is used to represent the pairs of integers, similar to the C struct.
3. **Sorting**: The sorting is done using Rust's `sort_by` method, which is safe and idiomatic.
4. **Cumulative Sums**: The cumulative sums are stored in a vector `s`, initialized to zero.
5. **Constants**: The constant `INF` is defined similarly to the C code.
6. **Indexing**: The Rust code uses 0-based indexing, so adjustments are made when accessing elements in the `pairs` vector.
7. **Output**: The output format is preserved with `println!`, ensuring the same behavior as the C `printf`.
