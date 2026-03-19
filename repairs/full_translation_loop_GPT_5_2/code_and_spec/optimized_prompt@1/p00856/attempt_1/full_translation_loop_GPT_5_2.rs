use std::f64;

const ONE_SIXTH: f64 = 1.0 / 6.0;
const DBL_MAX: f64 = f64::MAX;

fn solve(
    rest: usize,
    turn: i32,
    n: usize,
    sq: &[u8; 100],
    memo: &mut [[f64; 101]; 101],
) -> f64 {
    // C code indexes memo[rest][turn] with turn in [0..=100].
    // For turn <= 0, it returns 0.0 (unless rest==0 handled earlier).
    if memo[rest][turn as usize] != DBL_MAX {
        return memo[rest][turn as usize];
    }

    let ret = if rest == 0 {
        1.0
    } else if turn <= 0 {
        0.0
    } else if turn == 1 && rest <= 6 {
        ONE_SIXTH
    } else {
        let mut acc = 0.0;
        for i in 1..=6usize {
            // next_pos = abs(rest - i) in C (rest is non-negative).
            let next_pos = if rest >= i { rest - i } else { i - rest };

            // C uses sq[N-next_pos] (0 means normal, 'L' costs extra turn, 'B' resets to N).
            let idx = n - next_pos;
            match sq[idx] {
                0 => acc += ONE_SIXTH * solve(next_pos, turn - 1, n, sq, memo),
                b'L' => acc += ONE_SIXTH * solve(next_pos, turn - 2, n, sq, memo),
                b'B' => acc += ONE_SIXTH * solve(n, turn - 1, n, sq, memo),
                _ => {
                    // Unspecified; treat as empty.
                    acc += ONE_SIXTH * solve(next_pos, turn - 1, n, sq, memo)
                }
            }
        }
        acc
    };

    memo[rest][turn as usize] = ret;
    ret
}

fn main() {
    // Read all input as tokens.
    let input = std::io::read_to_end(std::io::stdin()).unwrap();
    let s = String::from_utf8_lossy(&input);
    let mut it = s.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n_i: i32 = n_opt.unwrap().parse().unwrap_or(0);
        let t_i: i32 = it.next().unwrap_or("0").parse().unwrap_or(0);
        let l_i: i32 = it.next().unwrap_or("0").parse().unwrap_or(0);
        let b_i: i32 = it.next().unwrap_or("0").parse().unwrap_or(0);

        // Termination condition: all four are zero.
        if n_i == 0 && t_i == 0 && l_i == 0 && b_i == 0 {
            break;
        }

        // Spec says: if invalid (e.g., negative), do not process and continue.
        // Also avoid out-of-bounds based on fixed array sizes in the original C.
        if n_i < 0 || t_i < 0 || l_i < 0 || b_i < 0 {
            // Consume obstacle positions to keep parsing aligned.
            for _ in 0..(l_i.max(0) as usize) {
                let _ = it.next();
            }
            for _ in 0..(b_i.max(0) as usize) {
                let _ = it.next();
            }
            continue;
        }

        let n = n_i as usize;
        let t = t_i as i32;
        let l = l_i as usize;
        let b = b_i as usize;

        // The C code assumes N<=100 and T<=100 due to array sizes.
        // If outside, treat as invalid per spec and skip (while consuming inputs).
        if n > 100 || t > 100 || l + b > n {
            for _ in 0..l {
                let _ = it.next();
            }
            for _ in 0..b {
                let _ = it.next();
            }
            continue;
        }

        let mut sq = [0u8; 100];
        let mut memo = [[DBL_MAX; 101]; 101];

        for _ in 0..l {
            if let Some(tok) = it.next() {
                if let Ok(pos_i) = tok.parse::<i32>() {
                    if pos_i >= 0 {
                        let pos = pos_i as usize;
                        if pos < 100 {
                            sq[pos] = b'L';
                        }
                    }
                }
            }
        }
        for _ in 0..b {
            if let Some(tok) = it.next() {
                if let Ok(pos_i) = tok.parse::<i32>() {
                    if pos_i >= 0 {
                        let pos = pos_i as usize;
                        if pos < 100 {
                            sq[pos] = b'B';
                        }
                    }
                }
            }
        }

        let ret = solve(n, t, n, &sq, &mut memo);
        out.push_str(&format!("{:.6}\n", ret));
    }

    print!("{out}");
}
