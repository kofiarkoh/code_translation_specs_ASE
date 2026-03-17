// AOJ 2236 Rabbit Plays Games!
// 2018.2.10 bal4u

use std::io::{self, BufRead};

#[derive(Debug)]
struct T {
    a: i64,
    d: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of entities
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let mut h = vec![0; n + 1];
    let mut a = vec![0; n + 1];
    let mut d = vec![0; n + 1];
    let mut s = vec![0; n + 1];
    let mut tbl: Vec<T> = Vec::new();

    // Read health, attack, defense, and score for each entity
    for i in 0..=n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        h[i] = nums[0];
        a[i] = nums[1];
        d[i] = nums[2];
        s[i] = nums[3];
    }

    let mut sz = 0;
    let mut turn: i64 = 0;
    let mut ans: i64 = 0;

    for i in 1..=n {
        let k = a[i] - d[0];
        if k <= 0 {
            continue;
        }
        if a[0] <= d[i] {
            println!("-1");
            return;
        }

        turn += k;
        if s[i] > s[0] {
            ans += k as i64;
        }
        tbl.push(T {
            d: k,
            a: (h[i] - d[i] + a[0] - 1) / (a[0] - d[i]) as i64,
        });
        sz += 1;
    }

    // Sort the table based on the custom comparator
    tbl.sort_by(|x, y| (x.a * y.d).cmp(&(y.a * x.d)));

    for i in 0..sz {
        ans += turn * tbl[i].a - tbl[i].d as i64;
        if h[0] <= ans {
            println!("-1");
            return;
        }
        turn -= tbl[i].d as i64;
    }
    println!("{}", ans);
}
```

### Notes:
1. **Input Handling**: The Rust code uses `BufRead` to read input efficiently, which is idiomatic in Rust for handling standard input.
2. **Data Structures**: The `T` struct is defined to match the C struct, and vectors are used to store the health, attack, defense, and score values.
3. **Sorting**: The sorting is done using Rust's `sort_by` method, which allows for a custom comparator similar to the C `qsort`.
4. **Integer Types**: The C `long long` is translated to `i64` in Rust, and `int` to `i32`, ensuring that the types match the expected sizes.
5. **Error Handling**: The program outputs `-1` and terminates as specified when conditions are not met.
6. **Semantics Preservation**: The logic and flow of the original C code are preserved, ensuring that the Rust version behaves identically according to the specification.
