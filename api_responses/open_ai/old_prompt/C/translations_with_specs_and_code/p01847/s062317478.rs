// AOJ 2747 Curtain
// Translated from C to Rust

use std::io::{self, BufRead};

const INF: i32 = 20002;

fn in_number() -> i32 {
    let mut n = 0;
    let mut c = std::io::stdin().bytes().next().unwrap().unwrap() as char;

    if c == '-' {
        c = std::io::stdin().bytes().next().unwrap().unwrap() as char;
        while c.is_digit(10) {
            n = 10 * n - (c as i32 - '0' as i32);
            c = std::io::stdin().bytes().next().unwrap().unwrap() as char;
        }
        return n;
    }

    while c.is_digit(10) {
        n = 10 * n + (c as i32 - '0' as i32);
        c = std::io::stdin().bytes().next().unwrap().unwrap() as char;
    }
    n
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut x = vec![0; 102];
    let mut y = vec![0; 102];

    loop {
        let n = in_number();
        if n == 0 {
            break; // Exit if N is 0
        }

        for i in 0..n {
            x[i as usize] = in_number();
            y[i as usize] = in_number();
        }
        x[n as usize] = x[0];
        y[n as usize] = y[0];

        let mut xmin = INF;
        let mut xmax = -INF;
        let mut ymin = INF;
        let mut ymax = -INF;

        for _ in 0..4 {
            let a = in_number();
            let b = in_number();
            if a < xmin { xmin = a; }
            if a > xmax { xmax = a; }
            if b < ymin { ymin = b; }
            if b > ymax { ymax = b; }
        }

        let mut win = 0i64;
        let mut cur = 0i64;

        for i in 0..n {
            if x[i as usize] == x[(i + 1) as usize] {
                cur += (x[i as usize] as i64) * (y[i as usize] - y[(i + 1) as usize]) as i64;
                let mut a = x[i as usize];
                if a > xmax { a = xmax; }
                if xmin > a { a = xmin; }
                let mut b = y[i as usize];
                if b > ymax { b = ymax; }
                if ymin > b { b = ymin; }
                let mut c = y[(i + 1) as usize];
                if c > ymax { c = ymax; }
                if ymin > c { c = ymin; }
                win += (a as i64) * (b - c) as i64;
            }
        }
        println!("{}", win - cur);
    }
}
```

### Notes:
1. **Input Handling**: The `in_number` function mimics the behavior of `getchar_unlocked()` for fast input reading. It reads bytes directly from stdin and converts them to integers.
2. **Loop Control**: The loop continues until `N` is 0, which is consistent with the C code's behavior.
3. **Array Initialization**: The arrays for `x` and `y` are initialized with a fixed size, as in the C code.
4. **Type Safety**: Rust's type system ensures that integer overflows are handled safely, and the use of `i64` for area calculations prevents overflow issues that could arise with larger values.
5. **Output**: The output format is preserved to match the specification, ensuring that the area is printed as a single long long integer.
