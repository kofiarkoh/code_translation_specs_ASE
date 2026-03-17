use std::cmp::max;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Item {
    w: i32,
    s: i32,
    v: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of items
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut items: Vec<Item> = Vec::with_capacity(n);

    // Read each item's weight, size, and value
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        items.push(Item { w: parts[0], s: parts[1], v: parts[2] });
    }

    // Sort items based on the combined metric of size and weight
    items.sort_by_key(|item| item.s + item.w);

    const M: usize = 20000;
    let mut dp: Vec<i64> = vec![0; M + 1];

    // Dynamic programming to compute the maximum value
    for item in items {
        for j in (0..=item.s).rev() {
            let k = (j + item.w) as usize;
            if k <= M {
                dp[k] = max(dp[k], dp[j as usize] + item.v as i64);
            }
        }
    }

    // Find the maximum value achievable
    let ans = dp.iter().cloned().max().unwrap_or(0);
    println!("{}", ans);
}
```

### Notes:
1. **Data Structures**: The `Item` struct is used to represent each item with its weight, size, and value.
2. **Input Handling**: The program reads input using `io::stdin` and processes it line by line, which is idiomatic in Rust.
3. **Sorting**: The items are sorted based on the combined metric of size and weight using `sort_by_key`.
4. **Dynamic Programming**: The dynamic programming logic is preserved, ensuring that the `dp` array is updated correctly.
5. **Output**: The maximum value is printed in the same format as specified.
6. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees.
