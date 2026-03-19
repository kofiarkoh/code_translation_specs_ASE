use std::f64;
use std::io::{self, Read};

const ONE_SIXTH: f64 = 1.0 / 6.0;

fn solve(
    rest: usize,
    turn: i32,
    n: usize,
    sq: &[u8; 100],
    memo: &mut [[f64; 101]; 101],
) -> f64 {
    // Spec terminal conditions
    if rest == 0 {
        return 1.0;
    }
    if turn <= 0 {
        return 0.0;
    }

    // Memoization only for turn in 0..=100 (per spec bounds).
    if (0..=100).contains(&turn) {
        let t = turn as usize;
        if memo[rest][t].is_finite() {
            return memo[rest][t];
        }
    }

    // C code has an extra shortcut: if turn==1 && rest<=6 => 1/6.
    // This is consistent with the spec transitions (only roll exactly rest succeeds).
    let ret = if turn == 1 && rest <= 6 {
        ONE_SIXTH
    } else {
        let mut acc = 0.0;
        for die in 1..=6usize {
            let next_pos = if rest >= die { rest - die } else { die - rest };

            // Board index mapping follows the C code: sq[N - next_pos]
            // (interpreting sq[] as positions from start, while state uses remaining distance).
            let idx = n - next_pos;
            match sq[idx] {
                0 => acc += ONE_SIXTH * solve(next_pos, turn - 1, n, sq, memo),
                b'L' => acc += ONE_SIXTH * solve(next_pos, turn - 2, n, sq, memo),
                b'B' => acc += ONE_SIXTH * solve(n, turn - 1, n, sq, memo),
                _ => {
                    // Unspecified; treat as normal.
                    acc += ONE_SIXTH * solve(next_pos, turn - 1, n, sq, memo)
                }
            }
        }
        acc
    };

    if (0..=100).contains(&turn) {
        memo[rest][turn as usize] = ret;
    }
    ret
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let t: i32 = it.next().unwrap().parse().unwrap();
        let l_cnt: usize = it.next().unwrap().parse().unwrap();
        let b_cnt: usize = it.next().unwrap().parse().unwrap();

        if n == 0 && t == 0 && l_cnt == 0 && b_cnt == 0 {
            break;
        }

        let mut sq = [0u8; 100];

        // Read lose-turn squares first.
        for _ in 0..l_cnt {
            let pos: usize = it.next().unwrap().parse().unwrap();
            if pos < 100 {
                sq[pos] = b'L';
            }
        }
        // Then back-to-start squares; per spec, B takes precedence if both listed.
        for _ in 0..b_cnt {
            let pos: usize = it.next().unwrap().parse().unwrap();
            if pos < 100 {
                sq[pos] = b'B';
            }
        }

        // Initialize memo with NaN as "unset" sentinel (safe equivalent of DBL_MAX check).
        let mut memo = [[f64::NAN; 101]; 101];

        let ans = solve(n, t, n, &sq, &mut memo);
        out.push_str(&format!("{:.6}\n", ans));
    }

    print!("{out}");
}
