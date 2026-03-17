// AOJ 3006: Chairs
// Translated from C to Rust

use std::io::{self, BufRead};

const MAX: usize = 100003;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of individuals (chairs)
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    let mut p = vec![0; n + 1]; // Chair preferences
    let mut hi = vec![0; n + 1]; // Count of preferences for each chair
    let mut q: Vec<Vec<usize>> = vec![Vec::new(); n + 1]; // Preferences storage
    let mut ans = vec![0; n + 1]; // Final assignments
    let mut sp = Vec::new(); // Stack for processing preferences

    // Read chair preferences
    for i in 1..=n {
        let a: usize = lines.next().unwrap().unwrap().parse().unwrap();
        p[i] = a;
        hi[a] += 1;
    }

    // Allocate space for preferences
    for i in 1..=n {
        q[i] = vec![0; hi[i]];
        hi[i] = 0; // Reset hi for usage as index
    }

    // Fill the preferences
    for i in 1..=n {
        let a = p[i];
        q[a][hi[a]] = i;
        hi[a] += 1;
    }

    let mut top = 0;

    // Process preferences to determine final assignments
    for _ in 0..2 {
        for i in 1..=n {
            while hi[i] > 0 {
                top += 1;
                hi[i] -= 1;
                sp.push(q[i][hi[i]]);
            }
            if ans[i] == 0 && top > 0 {
                top -= 1;
                ans[i] = sp.pop().unwrap();
            }
        }
    }

    // Output the final assignments
    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
```

### Notes:
1. **Input Handling**: The Rust code uses `io::stdin` for reading input, which is idiomatic and safe. The `unwrap()` calls are used under the assumption that input is valid as per the specification.
2. **Dynamic Arrays**: Rust's `Vec` is used to handle dynamic arrays instead of C-style arrays. This ensures memory safety and automatic management of memory.
3. **Preference Storage**: The `q` array is initialized as a vector of vectors, which allows for dynamic sizing based on the number of preferences.
4. **Stack Management**: The `sp` vector is used as a stack to manage the indices of individuals who prefer each chair.
5. **Output**: The output format is preserved exactly as specified, with each assignment printed on a new line.
6. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees.
