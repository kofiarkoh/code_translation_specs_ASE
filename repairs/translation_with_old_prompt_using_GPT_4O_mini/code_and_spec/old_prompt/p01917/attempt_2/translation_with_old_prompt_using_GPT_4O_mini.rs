use std::io::{self, BufRead};

const MAX_N: usize = 110;

fn f(n: usize, p: &[i32], x: &mut [i32]) {
    for i in 0..n {
        let mut cnt = 0;
        for j in 0..n {
            if p[j] > p[i] {
                cnt += 1;
            }
        }
        x[i] = p[cnt];
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse n");
    let mut x = vec![0; n];
    let mut a = vec![vec![0; n]; 3];

    for i in 0..n {
        x[i] = lines.next().unwrap().unwrap().parse().expect("Failed to parse x[i]");
    }
    
    for i in 0..n {
        let nums: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse a[kk][i]"))
            .collect();
        a[0][i] = nums[0];
        a[1][i] = nums[1];
        a[2][i] = nums[2];
    }

    let mut ans = 1000;

    for k in 0..3 {
        let mut s = vec![0; n];
        for kk in 0..3 {
            if kk != k {
                let mut t = vec![0; n];
                f(n, &a[kk], &mut t);
                for i in 0..n {
                    s[i] += t[i];
                }
            }
        }

        let ori = a[k][0];
        for i in 1..n {
            for d in 0..2 {
                if a[k][i] + d > ori {
                    a[k][0] = a[k][i] + d;
                    let mut t = vec![0; n];
                    f(n, &a[k], &mut t);
                    let mut cnt = 0;
                    for j in 0..n {
                        if s[j] + t[j] > s[0] + t[0] {
                            cnt += 1;
                        }
                    }
                    if cnt <= 7 {
                        ans = ans.min(a[k][0] - ori);
                    }
                }
            }
        }
        a[k][0] = ori;
    }

    if ans == 1000 {
        println!("Saiko");
    } else {
        println!("{}", ans);
    }
}
