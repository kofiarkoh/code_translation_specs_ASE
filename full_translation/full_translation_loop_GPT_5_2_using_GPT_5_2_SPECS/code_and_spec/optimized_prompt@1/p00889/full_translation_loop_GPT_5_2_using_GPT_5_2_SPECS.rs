use std::io::{self, Read};

fn main() {
    // Read entire stdin; if malformed/insufficient, stop (spec: no further output guarantees).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    // Precompute divisibility table used by the C code for q == 2 or q == 5.
    // tr[d][2] == 1 iff d even; tr[d][5] == 1 iff d == 0 or 5.
    let mut tr = [[false; 6]; 10];
    tr[0][2] = true;
    tr[2][2] = true;
    tr[4][2] = true;
    tr[6][2] = true;
    tr[8][2] = true;
    tr[0][5] = true;
    tr[5][5] = true;

    const HASHSIZ: usize = 300_017;

    loop {
        let n_opt = it.next().and_then(|s| s.parse::<i32>().ok());
        let s_opt = it.next().and_then(|s| s.parse::<i32>().ok());
        let w_opt = it.next().and_then(|s| s.parse::<i32>().ok());
        let q_opt = it.next().and_then(|s| s.parse::<i32>().ok());

        let (n, s, w, q) = match (n_opt, s_opt, w_opt, q_opt) {
            (Some(n), Some(s), Some(w), Some(q)) => (n, s, w, q),
            _ => break, // input failure => stop
        };

        if n <= 0 {
            break; // termination condition
        }

        let n_usize = n as usize;

        // Generate digit sequence a[0..n)
        let mut a = vec![0i32; n_usize];
        let mut g = s;
        for i in 0..n_usize {
            a[i] = (g / 7) % 10;
            if (g & 1) != 0 {
                g = (g >> 1) ^ w;
            } else {
                g >>= 1;
            }
        }

        let mut ans: i64 = 0;

        if q == 2 || q == 5 {
            // Count substrings divisible by 2 or 5:
            // A base-10 number is divisible by 2 iff last digit even; by 5 iff last digit 0 or 5.
            // Also enforce "subsequence does not start with digit 0".
            let mut k: i64 = 0; // number of possible starts so far with non-zero digit
            for i in 0..n_usize {
                if a[i] != 0 {
                    k += 1;
                }
                if tr[a[i] as usize][q as usize] {
                    ans += k;
                }
            }
        } else {
            // General case: compute suffix remainders b[i] = value(a[i..n-1]) mod q.
            // Then count pairs (i,j) with a[i]!=0 and value(a[i..j]) mod q == 0.
            // The C code uses a custom open-addressing hash keyed by (b[i]+1) to allow 0 sentinel.
            let q_i64 = q as i64;

            let mut b = vec![0i32; n_usize + 1];
            b[n_usize] = 0;
            let mut k: i64 = 1; // 10^0 mod q
            for idx in 0..n_usize {
                let i = n_usize - 1 - idx;
                let val = (k * (a[i] as i64) + (b[i + 1] as i64)) % q_i64;
                b[i] = val as i32;
                k = (k * 10) % q_i64;
            }

            // Hash table arrays: key b (0 means empty), value v (count).
            let mut key = vec![0i32; HASHSIZ];
            let mut val = vec![0i32; HASHSIZ];

            let mut lookup = |bkey: i32, key: &Vec<i32>, val: &Vec<i32>| -> i32 {
                let mut p = (bkey as i64).rem_euclid(HASHSIZ as i64) as usize;
                loop {
                    let kb = key[p];
                    if kb == 0 {
                        return 0;
                    }
                    if kb == bkey {
                        return val[p];
                    }
                    p += 1;
                    if p == HASHSIZ {
                        p = 0;
                    }
                }
            };

            let mut insert = |bkey: i32, key: &mut Vec<i32>, val: &mut Vec<i32>| {
                let mut p = (bkey as i64).rem_euclid(HASHSIZ as i64) as usize;
                loop {
                    let kb = key[p];
                    if kb == 0 {
                        key[p] = bkey;
                        val[p] = 1;
                        return;
                    }
                    if kb == bkey {
                        val[p] += 1;
                        return;
                    }
                    p += 1;
                    if p == HASHSIZ {
                        p = 0;
                    }
                }
            };

            for i in 0..n_usize {
                if a[i] != 0 {
                    insert(b[i] + 1, &mut key, &mut val);
                }
                ans += lookup(b[i + 1] + 1, &key, &val) as i64;
            }
        }

        println!("{}", ans);
    }
}
