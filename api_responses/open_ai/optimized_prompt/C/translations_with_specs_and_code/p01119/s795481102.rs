// AOJ 1634 Balance Scale
// bal4u 2019.8.12

use std::collections::HashSet;
use std::io::{self, BufRead};

const INF: i32 = i32::MAX;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        let m: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        let mut a = Vec::with_capacity(n);
        for _ in 0..n {
            let value: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
            a.push(value);
        }

        let mut w = Vec::with_capacity(m);
        for _ in 0..m {
            let weight: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
            w.push(weight);
        }

        let mut q = vec![0; 60000]; // 3^10
        let mut end = 1;

        for &weight in &w {
            let e = end;
            for j in 0..e {
                q[end] = q[j] + weight;
                end += 1;
                q[end] = q[j] - weight;
                end += 1;
            }
        }

        let mut hash_set1 = HashSet::new();
        for i in 0..end {
            if q[i] > 0 {
                hash_set1.insert(q[i]);
            }
        }

        let mut x = 0;
        let mut ans = 0;

        for &value in &a {
            if !hash_set1.contains(&value) {
                if x == 0 {
                    let mut t;
                    ans = INF;
                    let mut hash_set2 = HashSet::new();
                    for j in 0..end {
                        t = (value - q[j]).abs();
                        if t < ans {
                            ans = t;
                        }
                        hash_set2.insert(t);
                    }
                    x = 1;
                } else {
                    let mut t;
                    let mut r = 0;
                    x += 1;
                    ans = INF;
                    for j in 0..end {
                        t = (value - q[j]).abs();
                        if hash_set2.contains(&(t, x)) {
                            if t < ans {
                                ans = t;
                            }
                            r = 1;
                        }
                    }
                    if r == 0 {
                        ans = -1;
                        break;
                    }
                }
            }
        }

        println!("{}", if ans == INF { -1 } else { ans });
    }
}
```

### Notes:
1. **Input Handling**: The Rust code uses `io::stdin` to read input, which is idiomatic for handling standard input in Rust.
2. **Data Structures**: The C code's hash tables are replaced with `HashSet` for simplicity and safety, which provides average O(1) time complexity for insertions and lookups.
3. **Absolute Difference Calculation**: The absolute difference is calculated using Rust's built-in `abs()` method.
4. **Output**: The output is formatted to match the C code's behavior, ensuring that the newline is preserved.
5. **Error Handling**: The Rust code assumes valid input as per the specification, similar to the C code.
6. **Safety**: The translation avoids any use of `unsafe` code, adhering to Rust's safety guarantees.
