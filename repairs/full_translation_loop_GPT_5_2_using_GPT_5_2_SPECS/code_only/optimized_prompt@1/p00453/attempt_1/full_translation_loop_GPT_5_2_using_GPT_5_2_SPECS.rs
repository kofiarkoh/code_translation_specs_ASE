use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 tokens (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        let m: i32 = it.next().unwrap().parse().unwrap();

        // while(scanf(...), n||m)
        if n == 0 && m == 0 {
            break;
        }

        // Arrays sized exactly as in C.
        let mut ic = [0i32; 150];
        let mut is_arr = [[[0i32; 10]; 150]; 2];

        // C has: int c[80][150][11]={0}; then sets most entries to 2100000000
        // We'll build it directly.
        let mut c = vec![[[0i32; 11]; 150]; 80];

        // Initialize: for i<80, for j=1..149, for k<11 => 2100000000
        for i in 0..80usize {
            for j in 1..150usize {
                for k in 0..11usize {
                    c[i][j][k] = 2_100_000_000i32;
                }
            }
        }
        // for i<11: c[1][1][i]=0;
        for i in 0..11usize {
            c[1][1][i] = 0;
        }

        // Read input structure.
        let n_usize = n as usize;
        for i in 0..n_usize {
            ic[i] = it.next().unwrap().parse().unwrap();
            let cnt = ic[i] as usize;
            for j in 0..cnt {
                is_arr[0][i][j] = it.next().unwrap().parse().unwrap();
                is_arr[1][i][j] = it.next().unwrap().parse().unwrap();
            }
        }

        // DP transitions.
        // Note: C uses signed int arithmetic; overflow is UB in C, but typical behavior is
        // two's-complement wrap. We use wrapping_* to emulate that common behavior.
        let m_usize = m as usize;
        for i in 0..=m_usize {
            for j in 0..(n_usize.saturating_sub(1)) {
                let icj = ic[j] as usize;
                for k in 0..icj {
                    // to j+1
                    let icj1 = ic[j + 1] as usize;
                    for l in 0..icj1 {
                        let a1 = is_arr[1][j][k];
                        let b1 = is_arr[1][j + 1][l];
                        let a0 = is_arr[0][j][k];
                        let b0 = is_arr[0][j + 1][l];

                        // d=(a1+b1)*abs(a0-b0)
                        let sum = a1.wrapping_add(b1);
                        let diff = a0.wrapping_sub(b0);
                        // abs(INT_MIN) is UB in C; emulate typical two's-complement by wrapping.
                        let adiff = if diff < 0 { diff.wrapping_neg() } else { diff };
                        let d = sum.wrapping_mul(adiff);

                        let cur = c[i][j][k];
                        let cand = cur.wrapping_add(d);
                        if c[i][j + 1][l] > cand {
                            c[i][j + 1][l] = cand;
                        }
                    }

                    if j + 2 == n_usize {
                        continue;
                    }

                    // to j+2 with i+1
                    let icj2 = ic[j + 2] as usize;
                    for l in 0..icj2 {
                        let a1 = is_arr[1][j][k];
                        let b1 = is_arr[1][j + 2][l];
                        let a0 = is_arr[0][j][k];
                        let b0 = is_arr[0][j + 2][l];

                        let sum = a1.wrapping_add(b1);
                        let diff = a0.wrapping_sub(b0);
                        let adiff = if diff < 0 { diff.wrapping_neg() } else { diff };
                        let d = sum.wrapping_mul(adiff);

                        let cur = c[i][j][k];
                        let cand = cur.wrapping_add(d);
                        if c[i + 1][j + 2][l] > cand {
                            c[i + 1][j + 2][l] = cand;
                        }
                    }
                }
            }
        }

        // Compute min as in C.
        let mut minv: i32 = 2_100_000_210i32;
        for i in 0..=m_usize {
            let last = n_usize - 1;
            let ic_last = ic[last] as usize;
            for j in 0..ic_last {
                let v = c[i][last][j];
                if minv > v {
                    minv = v;
                }
            }
            if m_usize == i {
                continue;
            }
            let prev = n_usize - 2;
            let ic_prev = ic[prev] as usize;
            for j in 0..ic_prev {
                let v = c[i][prev][j];
                if minv > v {
                    minv = v;
                }
            }
        }

        out.push_str(&format!("{minv}\n"));
    }

    print!("{out}");
}
