use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let h: usize = params.next().unwrap().parse().unwrap();
    let w: usize = params.next().unwrap().parse().unwrap();
    let k: usize = params.next().unwrap().parse().unwrap();

    let mut s = vec![vec!['0'; 1002]; h + 1]; // Adjust size based on h
    for i in 1..=h {
        let line = lines.next().unwrap().unwrap();
        for (j, c) in line.chars().enumerate() {
            s[i][j + 1] = c; // Fill from index 1
        }
    }

    let mut t = vec![0; h + 1]; // Adjust size based on h
    let mut b = vec![0; h + 1]; // Adjust size based on h
    let mut flag = vec![0; h + 1]; // Adjust size based on h
    let mut count = vec![0; h + 1]; // Adjust size based on h
    let mut ans = usize::MAX; // Use usize::MAX for maximum value
    let mut tmp;

    flag[h] = 1;
    while flag[h + 1] == 0 {
        t[0] = 1;
        let mut i = 0;
        while t[i] <= h {
            count[i] = 0;
            b[i] = t[i];
            while b[i] <= h && flag[b[i]] == 0 {
                b[i] += 1;
            }
            t[i + 1] = b[i] + 1;
            i += 1;
        }

        let mut j = 1;
        tmp = i - 1;
        while j <= w {
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
                let mut i = 0;
                while t[i] <= h {
                    count[i] = 0;
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
                    break;
                } else {
                    tmp += 1;
                }
            }
            j += 1;
        }
        if j > w && tmp < ans {
            ans = tmp;
        }

        let mut i = 1;
        while i <= h && flag[i] == 1 {
            i += 1;
        }
        if i > 1 {
            flag[i - 1] = 1;
        }
        while i >= 1 {
            flag[i - 1] = 0;
            i -= 1;
        }
    }

    println!("{}", ans);
}
