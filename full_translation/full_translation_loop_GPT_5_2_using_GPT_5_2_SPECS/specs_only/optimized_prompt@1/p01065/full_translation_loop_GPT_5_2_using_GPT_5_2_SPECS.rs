use std::io::{self, Read};

fn main() {
    // Fixed internal capacities implied by the specification.
    const MAX_OFFERS: usize = 100;
    const MAX_A: usize = 300;
    const MAX_B: usize = 300;

    // Quantity B is tracked in a shifted index space to allow negative intermediate values.
    // Supported actual range is approximately [-300, 300], so index range is [0, 600].
    const B_SHIFT: isize = 300;
    const B_SIZE: usize = 601; // -300..=300

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let limit_a: usize = it.next().unwrap().parse().unwrap();
    let limit_b: isize = it.next().unwrap().parse::<usize>().unwrap() as isize;

    // DP over (A, B) storing max value; unreachable = -1.
    // We only need two layers (prefix invariant).
    let mut dp = vec![vec![-1i64; B_SIZE]; MAX_A + 1];
    let mut next = vec![vec![-1i64; B_SIZE]; MAX_A + 1];

    // Initial state: A=0, B=0, value=0.
    dp[0][(0 + B_SHIFT) as usize] = 0;

    for _ in 0..n.min(MAX_OFFERS) {
        let inc_a: usize = it.next().unwrap().parse().unwrap();
        let dec_b: isize = it.next().unwrap().parse::<usize>().unwrap() as isize;
        let inc_b: isize = it.next().unwrap().parse::<usize>().unwrap() as isize;
        let inc_value: i64 = it.next().unwrap().parse::<usize>().unwrap() as i64;

        // Reset next layer to unreachable.
        for a in 0..=MAX_A {
            next[a].fill(-1);
        }

        for a in 0..=MAX_A {
            for bi in 0..B_SIZE {
                let cur = dp[a][bi];
                if cur < 0 {
                    continue;
                }
                let b_actual = bi as isize - B_SHIFT;

                // Unchanged.
                if cur > next[a][bi] {
                    next[a][bi] = cur;
                }

                // Mode 1: (A + incA, B - decB, value), only if A + incA <= limit A.
                // The specification only guards by limit A; B bounds are assumed by preconditions.
                if a + inc_a <= limit_a {
                    let na = a + inc_a;
                    if na <= MAX_A {
                        let nb = b_actual - dec_b;
                        let nbi = nb + B_SHIFT;
                        if (0..B_SIZE as isize).contains(&nbi) {
                            let nbiu = nbi as usize;
                            if cur > next[na][nbiu] {
                                next[na][nbiu] = cur;
                            }
                        }
                    }
                }

                // Mode 2: (A, B + incB, value + incValue).
                {
                    let nb = b_actual + inc_b;
                    let nbi = nb + B_SHIFT;
                    if (0..B_SIZE as isize).contains(&nbi) {
                        let nbiu = nbi as usize;
                        let nv = cur + inc_value;
                        if nv > next[a][nbiu] {
                            next[a][nbiu] = nv;
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut dp, &mut next);
    }

    // Final selection: A <= limit A and B <= limit B.
    let mut ans: i64 = 0;
    let a_max = limit_a.min(MAX_A);
    for a in 0..=a_max {
        for bi in 0..B_SIZE {
            let v = dp[a][bi];
            if v < 0 {
                continue;
            }
            let b_actual = bi as isize - B_SHIFT;
            if b_actual <= limit_b && v > ans {
                ans = v;
            }
        }
    }

    // Output is a single non-negative integer; if nothing feasible, remains 0.
    println!("{}", ans);
}
