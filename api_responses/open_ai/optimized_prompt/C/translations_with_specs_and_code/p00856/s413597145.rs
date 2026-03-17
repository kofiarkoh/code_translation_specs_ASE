use std::io::{self, BufRead};
use std::f64;

const ONE_SIXTH: f64 = 1.0 / 6.0;

fn myabs(x: i32) -> i32 {
    if x >= 0 { x } else { -x }
}

fn solve(rest: usize, turn: usize, memo: &mut Vec<Vec<f64>>, sq: &[i32]) -> f64 {
    if memo[rest][turn] != f64::MAX {
        return memo[rest][turn];
    } else if rest == 0 {
        return 1.0;
    } else if turn == 0 {
        return 0.0;
    } else if turn == 1 && rest <= 6 {
        return ONE_SIXTH;
    } else {
        let mut ret = 0.0;
        for i in 1..=6 {
            let next_pos = myabs(rest as i32 - i);
            match sq[sq.len() - next_pos as usize] {
                0 => ret += ONE_SIXTH * solve(next_pos as usize, turn - 1, memo, sq),
                'L' => ret += ONE_SIXTH * solve(next_pos as usize, turn - 2, memo, sq),
                'B' => ret += ONE_SIXTH * solve(sq.len() - 1, turn - 1, memo, sq),
                _ => {}
            }
        }
        memo[rest][turn] = ret;
        return ret;
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = reader.lines().map(|line| line.unwrap());

    loop {
        let line = input.next().unwrap();
        let mut parts = line.split_whitespace();
        let n: i32 = parts.next().unwrap().parse().unwrap();
        let t: i32 = parts.next().unwrap().parse().unwrap();
        let l: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();

        if n == 0 && t == 0 && l == 0 && b == 0 {
            break;
        }

        let mut sq = vec![0; 101];
        let mut memo = vec![vec![f64::MAX; 101]; 101];

        for _ in 0..l {
            let l_pos: usize = input.next().unwrap().parse().unwrap();
            sq[l_pos] = 'L' as i32; // Store 'L' as its ASCII value
        }
        for _ in 0..b {
            let b_pos: usize = input.next().unwrap().parse().unwrap();
            sq[b_pos] = 'B' as i32; // Store 'B' as its ASCII value
        }

        let ret = solve(n as usize, t as usize, &mut memo, &sq);
        println!("{:.6}", ret);
    }
}
```

### Notes:
1. **Input Handling**: The Rust code uses `io::stdin` to read input, which is idiomatic for Rust. It processes input line by line.
2. **Memoization**: The memoization structure is implemented as a 2D vector initialized with `f64::MAX`, similar to the C code.
3. **Special Squares**: The special squares are stored as ASCII values ('L' and 'B') in the `sq` array, which is a direct translation from the C code.
4. **Functionality**: The `solve` function is translated to maintain the same logic and flow as the C version, ensuring that the semantics are preserved.
5. **Output Formatting**: The output is formatted to six decimal places to match the expected output format.
6. **Safety**: The code avoids `unsafe` blocks, adhering to Rust's safety guarantees.
