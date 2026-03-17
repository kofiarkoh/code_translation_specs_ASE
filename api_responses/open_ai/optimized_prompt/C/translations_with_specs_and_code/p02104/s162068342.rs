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
    
    // Read the identifiers and count occurrences
    for i in 1..=n {
        let a: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        p[i] = a;
        hi[a] += 1;
    }

    // Allocate space for each identifier
    for i in 1..=n {
        q[i] = vec![0; hi[i]];
    }

    // Reset hi for the next use
    hi.fill(0);
    
    // Fill the q array with indices
    for i in 1..=n {
        let a = p[i];
        q[a][hi[a]] = i;
        hi[a] += 1;
    }

    let mut top = 0;

    // Rearranging identifiers according to the specified logic
    for _ in 0..2 {
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
2. **Dynamic Arrays**: Instead of using fixed-size arrays, Rust's `Vec` is used to handle dynamic sizes, which is safer and more idiomatic.
3. **Memory Management**: The original C code uses `malloc`, which is replaced by Rust's ownership model and automatic memory management.
4. **Error Handling**: The C code does not specify error handling, so the Rust code assumes valid input as per the specification.
5. **Preservation of Semantics**: The logic and flow of the original C code are preserved, ensuring that the output matches the expected rearrangement of identifiers.
