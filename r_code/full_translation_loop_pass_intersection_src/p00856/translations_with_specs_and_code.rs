use std::io::{self, Read};

const ONE_SIXTH: f64 = 1.0 / 6.0;

fn solve(
    rest: usize,
    turn: i32,
    n: usize,
    sq: &[u8; 101],
    memo: &mut [Vec<Option<f64>>; 101],
) -> f64 {
    if turn >= 0 {
        if let Some(v) = memo[rest][turn as usize] {
            return v;
        }
    }

    let ret = if rest == 0 {
        1.0
    } else if turn <= 0 {
        0.0
    } else if turn == 1 && rest <= 6 {
        // Base case per spec/C: ignore special squares here.
        ONE_SIXTH
    } else {
        let mut acc = 0.0;
        for d in 1..=6usize {
            let next_rest = if rest >= d { rest - d } else { d - rest }; // |rest - d|
            let pos = n - next_rest; // position from start
            match sq[pos] {
                0 => acc += ONE_SIXTH * solve(next_rest, turn - 1, n, sq, memo),
                b'L' => acc += ONE_SIXTH * solve(next_rest, turn - 2, n, sq, memo),
                b'B' => acc += ONE_SIXTH * solve(n, turn - 1, n, sq, memo),
                _ => {} // not expected
            }
        }
        acc
    };

    if turn >= 0 {
        memo[rest][turn as usize] = Some(ret);
    }
    ret
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let Some(ns) = it.next() else { break };
        let n: usize = ns.parse().unwrap();
        let t: i32 = it.next().unwrap().parse().unwrap();
        let lcnt: usize = it.next().unwrap().parse().unwrap();
        let bcnt: usize = it.next().unwrap().parse().unwrap();

        if n == 0 && t == 0 && lcnt == 0 && bcnt == 0 {
            break;
        }

        let mut sq = [0u8; 101];

        // Fixed-size memo like the C version (101x101), initialized to "unset".
        let mut memo: [Vec<Option<f64>>; 101] = std::array::from_fn(|_| vec![None; 101]);

        for _ in 0..lcnt {
            let pos: usize = it.next().unwrap().parse().unwrap();
            sq[pos] = b'L';
        }
        for _ in 0..bcnt {
            let pos: usize = it.next().unwrap().parse().unwrap();
            // Back-to-start overwrites lose-extra-turn if both specified.
            sq[pos] = b'B';
        }

        let ans = solve(n, t, n, &sq, &mut memo);

        // C uses printf("%lf\n", ret) which prints 6 digits after the decimal by default.
        println!("{:.6}", ans);
    }
}
