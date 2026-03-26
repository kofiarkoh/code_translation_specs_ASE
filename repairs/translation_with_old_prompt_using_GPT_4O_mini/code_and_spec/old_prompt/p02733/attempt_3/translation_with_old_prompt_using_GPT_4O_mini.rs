use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read H, W, K
    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let h: usize = params.next().unwrap().parse().unwrap();
    let w: usize = params.next().unwrap().parse().unwrap();
    let k: usize = params.next().unwrap().parse().unwrap();

    // Read the grid
    let mut s = vec![vec!['0'; w + 2]; h + 1]; // +2 for 1-based indexing and padding
    for i in 1..=h {
        let line = lines.next().unwrap().unwrap();
        for j in 1..=w {
            s[i][j] = line.chars().nth(j - 1).unwrap();
        }
    }

    let mut flag = vec![0; h + 2];
    flag[h] = 1;
    let mut ans = usize::MAX;

    while flag[h + 1] == 0 {
        let mut t = vec![1];
        let mut b = vec![0; h + 1];
        let mut count = vec![0; h + 1];

        for i in 0..=h {
            count[i] = 0;
            b[i] = t[i];
            while b[i] <= h && flag[b[i]] == 0 {
                b[i] += 1;
            }
            t.push(b[i] + 1);
        }

        let mut tmp = 0;
        for j in 1..=w {
            let mut i = 0;
            while t[i] <= h {
                for k in t[i]..=b[i] {
                    if s[k][j] == '1' {
                        count[i] += 1;
                    }
                }
                if count[i] > k {
                    break;
                }
                i += 1;
            }

            if t[i] <= h {
                for i in 0..=h {
                    count[i] = 0;
                    for k in t[i]..=b[i] {
                        if s[k][j] == '1' {
                            count[i] += 1;
                        }
                    }
                    if count[i] > k {
                        break;
                    }
                }
                if t[i] <= h {
                    break;
                } else {
                    tmp += 1;
                }
            }
        }

        if tmp < ans {
            ans = tmp;
        }

        let mut i = 1;
        while i <= h && flag[i] == 1 {
            i += 1;
        }
        for j in (1..=i).rev() {
            if j <= h {
                flag[j] = 0;
            }
        }
        if i <= h {
            flag[i] = 1;
        }
    }

    println!("{}", ans);
}
