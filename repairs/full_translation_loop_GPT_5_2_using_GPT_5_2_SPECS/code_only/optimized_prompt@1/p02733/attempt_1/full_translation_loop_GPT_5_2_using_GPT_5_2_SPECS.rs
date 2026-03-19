use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();
    let k_limit: i32 = it.next().unwrap().parse().unwrap();

    // C uses 1-based indexing for both rows and columns:
    // char S[12][1002]; and reads into &(S[i][1]) for i=1..H.
    // We'll store as bytes with an extra leading dummy column at index 0.
    let mut s: Vec<Vec<u8>> = vec![vec![b'0'; w + 2]; h + 2];
    for i in 1..=h {
        let row = it.next().unwrap().as_bytes();
        for j in 1..=w {
            s[i][j] = row[j - 1];
        }
    }

    // Arrays sized to cover indices up to H+1 (C uses flag[H+1] as sentinel check).
    // In the original constraints H<=10, but we follow the C fixed sizes conceptually.
    let mut t: Vec<usize> = vec![0; h + 3];
    let mut b: Vec<usize> = vec![0; h + 3];
    let mut flag: Vec<i32> = vec![0; h + 3];
    let mut count: Vec<i32> = vec![0; h + 3];

    let mut ans: i32 = 1 << 30;

    flag[h] = 1;
    while flag[h + 1] == 0 {
        t[0] = 1;

        // Build horizontal groups based on flag[] cut positions.
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

        // tmp starts as number of horizontal cuts: (number of groups - 1)
        let mut tmp: i32 = i as i32 - 1;

        // Scan columns, adding vertical cuts when needed.
        let mut j: usize = 1;
        while j <= w {
            // Add this column to current segment counts.
            let mut ii: usize = 0;
            while t[ii] <= h {
                let mut kk = t[ii];
                while kk <= b[ii] {
                    if s[kk][j] == b'1' {
                        count[ii] += 1;
                    }
                    kk += 1;
                }
                if count[ii] > k_limit {
                    break;
                }
                ii += 1;
            }

            if t[ii] <= h {
                // Need to cut before this column: reset counts and try this column alone.
                let mut iii: usize = 0;
                while t[iii] <= h {
                    count[iii] = 0;
                    let mut kk = t[iii];
                    while kk <= b[iii] {
                        if s[kk][j] == b'1' {
                            count[iii] += 1;
                        }
                        kk += 1;
                    }
                    if count[iii] > k_limit {
                        break;
                    }
                    iii += 1;
                }

                if t[iii] <= h {
                    // Even a single column violates K => this partitioning is impossible.
                    break;
                } else {
                    // Place a vertical cut (increment tmp) and continue with this column as first.
                    tmp += 1;
                }
            }

            j += 1;
        }

        if j > w && tmp < ans {
            ans = tmp;
        }

        // Next flag pattern generation (mimics the C code exactly).
        // for (i = 1; i <= H && flag[i] == 1; i++);
        let mut idx: usize = 1;
        while idx <= h && flag[idx] == 1 {
            idx += 1;
        }
        // for (flag[i--] = 1; i >= 1; flag[i--] = 0);
        // Careful with usize underflow; emulate the C loop structure.
        if idx <= h {
            flag[idx] = 1;
            let mut p: isize = idx as isize - 1;
            while p >= 1 {
                flag[p as usize] = 0;
                p -= 1;
            }
        } else {
            // If idx == h+1, the C code would write flag[h+1]=1 and terminate next check.
            flag[h + 1] = 1;
        }
    }

    print!("{ans}\n");
    // stdout flush is implicit on program exit; C calls fflush(stdout) but not required here.
}
