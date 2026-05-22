use std::io::{self, Read};

fn solve_instance(n: usize, t: i32, board: &[u8]) -> f64 {
    // Memo table indexed by (r, t). Bounds are assumed valid per spec (<= 100).
    // Use NaN as "uncomputed" sentinel.
    let mut memo = vec![vec![f64::NAN; (t.max(0) as usize) + 1]; n + 1];

    fn rec(n: usize, r: usize, t: i32, board: &[u8], memo: &mut [Vec<f64>]) -> f64 {
        // Success state
        if r == 0 {
            return 1.0;
        }
        // Failure state
        if t <= 0 {
            return 0.0;
        }
        // Additional base-case behavior (special squares ignored)
        if t == 1 && r <= 6 {
            return 1.0 / 6.0;
        }

        let tu = t as usize;
        if !memo[r][tu].is_nan() {
            return memo[r][tu];
        }

        let mut acc = 0.0f64;
        for d in 1usize..=6 {
            // Bounce rule: r' = |r - d|
            let rp = if r >= d { r - d } else { d - r };
            // pos = N - r'
            let pos = n - rp;

            // board[pos]: 0 normal, 1 lose-extra-turn, 2 back-to-start
            let val = match board[pos] {
                1 => rec(n, rp, t - 2, board, memo),
                2 => rec(n, n, t - 1, board, memo),
                _ => rec(n, rp, t - 1, board, memo),
            };
            acc += val;
        }

        let res = acc / 6.0;
        memo[r][tu] = res;
        res
    }

    rec(n, n, t, board, &mut memo)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let Some(ns) = it.next() else { break };
        let n: usize = ns.parse().unwrap();
        let t: i32 = it.next().unwrap().parse().unwrap();
        let l: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();

        if n == 0 && t == 0 && l == 0 && b == 0 {
            break;
        }

        // board positions are measured from the start, valid indices assumed.
        // 0 = normal, 1 = lose-extra-turn, 2 = back-to-start
        let mut board = vec![0u8; n + 1];

        for _ in 0..l {
            let pos: usize = it.next().unwrap().parse().unwrap();
            board[pos] = 1;
        }
        for _ in 0..b {
            let pos: usize = it.next().unwrap().parse().unwrap();
            // Back-to-start overwrites lose-extra-turn if both specified.
            board[pos] = 2;
        }

        let ans = solve_instance(n, t, &board);

        // The reference output matches typical "%lf" usage with fixed 6 decimals in this problem.
        // Use 6 digits after the decimal point to preserve observable I/O behavior.
        out.push_str(&format!("{:.6}\n", ans));
    }

    print!("{out}");
}
