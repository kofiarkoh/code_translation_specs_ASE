// AOJ 3006: Chairs
// Translated from C to Rust

use std::io::{self, BufRead};

const MAX: usize = 100003;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of identifiers
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let mut p = vec![0; MAX];
    let mut hi = vec![0; MAX];
    let mut q: Vec<Vec<usize>> = vec![Vec::new(); MAX];
    let mut ans = vec![0; MAX];
    let mut sp = Vec::new();
    let mut top = 0;

    // Read chair identifiers
    for i in 1..=n {
        let a: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        p[i] = a;
        hi[a] += 1;
    }

    // Allocate space for q based on hi counts
    for i in 1..=n {
        q[i] = vec![0; hi[i]];
    }

    // Reset hi for the next use
    hi.fill(0);

    // Fill q with positions of identifiers
    for i in 1..=n {
        let a = p[i];
        q[a][hi[a]] = i;
        hi[a] += 1;
    }

    // Process identifiers to generate the answer
    for j in 0..2 {
        for i in 1..=n {
            while hi[i] > 0 {
                top += 1;
                sp.push(q[i][hi[i] - 1]);
                hi[i] -= 1;
            }
            if ans[i] == 0 && top > 0 {
                top -= 1;
                ans[i] = sp.pop().unwrap();
            }
        }
    }

    // Output the results
    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
```

### Notes:
1. **Input Handling**: The Rust code uses `BufRead` for efficient input reading, which is idiomatic in Rust.
2. **Dynamic Arrays**: The C code's use of `malloc` is replaced with Rust's `Vec`, which handles memory safely and automatically.
3. **Indexing**: The C code uses 1-based indexing, which is preserved in the Rust translation.
4. **Error Handling**: The original C code does not specify error handling, so the Rust code assumes valid input as per the specification.
5. **Memory Management**: Rust's ownership model eliminates the need for manual memory management, making the code safer and more idiomatic.
