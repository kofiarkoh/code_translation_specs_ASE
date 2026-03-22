use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();
    let k_limit: i32 = it.next().unwrap().parse().unwrap();

    // C uses 1-based indexing for rows and columns:
    // S[12][1002], and reads into &(S[i][1]) with scanf("%s", ...)
    // We'll store as bytes with an extra leading dummy column at index 0.
    let mut s: Vec<Vec<u8>> = vec![vec![b'0'; w + 2]; h + 2];
    for i in 1..=h {
        let row = it.next().unwrap().as_bytes();
        for j in 1..=w {
            s[i][j] = row[j - 1];
        }
    }

    // Arrays sized to cover indices up to H+1 as in C (flag[H+1] is read).
    let mut t = vec![0usize; h + 3];
    let mut b = vec![0usize; h + 3];
    let mut flag = vec![0i32; h + 3];
    let mut count = vec![0i32; h + 3];

    let mut ans: i32 = 1 << 30;

    flag[h] = 1;
    while flag[h + 1] == 0 {
        t[0] = 1;

        // Build segments [t[i], b[i]] based on cut flags.
        let mut i: usize = 0;
        while t[i] <= h {
            count[i] = 0;
            b[i] = t[i];
            while flag[b[i]] == 0 {
                b[i] += 1;
            }
            t[i + 1] = b[i] + 1;
            i += 1;
        }

        // tmp starts as number of horizontal cuts = (number of segments - 1)
        let mut tmp: i32 = i as i32 - 1;

        // Process columns, greedily adding vertical cuts when needed.
        let mut j: usize = 1;
        while j <= w {
            // Try to extend current vertical block with column j
            let mut ii: usize = 0;
            while t[ii] <= h {
                for kk in t[ii]..=b[ii] {
                    if s[kk][j] == b'1' {
                        count[ii] += 1;
                    }
                }
                if count[ii] > k_limit {
                    break;
                }
                ii += 1;
            }

            if t[ii] <= h {
                // Need a vertical cut before column j: reset counts and try column j alone
                let mut iii: usize = 0;
                while t[iii] <= h {
                    count[iii] = 0;
                    for kk in t[iii]..=b[iii] {
                        if s[kk][j] == b'1' {
                            count[iii] += 1;
                        }
                    }
                    if count[iii] > k_limit {
                        break;
                    }
                    iii += 1;
                }

                if t[iii] <= h {
                    // Even a single column violates K for some segment: this partition invalid
                    break;
                } else {
                    // Place a vertical cut (increment tmp) and continue with next column
                    tmp += 1;
                }
            }

            j += 1;
        }

        if j > w && tmp < ans {
            ans = tmp;
        }

        // Next flag pattern (binary-like increment over positions 1..H),
        // matching the C code's exact update logic.
        let mut p: usize = 1;
        while p <= h && flag[p] == 1 {
            p += 1;
        }
        // In the original C, p is guaranteed <= H+1 because flag[H]=1 initially
        // and the loop terminates when flag[H+1] becomes 1.
        if p == 0 {
            // unreachable
        } else {
            p -= 1;
            flag[p] = 1;
            while p >= 1 {
                flag[p] = 0;
                if p == 1 {
                    break;
                }
                p -= 1;
            }
        }
    }

    print!("{ans}\n");
}
