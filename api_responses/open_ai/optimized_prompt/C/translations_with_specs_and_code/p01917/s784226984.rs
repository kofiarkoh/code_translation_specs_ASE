use std::io::{self, BufRead};

const MAX_SIZE: usize = 110;

fn compute_min_difference(n: usize, x: &[i32], a: &[[i32; MAX_SIZE]; 3]) -> String {
    let mut s = [0; MAX_SIZE];
    let mut t = [0; MAX_SIZE];
    let mut ans = 1000;

    for k in 0..3 {
        s.iter_mut().for_each(|v| *v = 0);
        
        for kk in 0..3 {
            if kk != k {
                f(&a[kk], n, &mut t);
                for i in 0..n {
                    s[i] += t[i];
                }
            }
        }

        let ori = a[k][0];
        for i in 1..n {
            for d in 0..2 {
                if a[k][i] + d > ori {
                    let mut modified_a = a[k];
                    modified_a[0] = a[k][i] + d;
                    f(&modified_a, n, &mut t);
                    let cnt = (0..n).filter(|&j| s[j] + t[j] > s[0] + t[0]).count();
                    if cnt <= 7 {
                        ans = ans.min(modified_a[0] - ori);
                    }
                }
            }
        }
    }

    if ans == 1000 {
        "Saiko".to_string()
    } else {
        ans.to_string()
    }
}

fn f(p: &[i32], n: usize, t: &mut [i32]) {
    for i in 0..n {
        let cnt = (0..n).filter(|&j| p[j] > p[i]).count();
        t[i] = p[cnt];
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut x = vec![0; n];
    for i in 0..n {
        x[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    let mut a = [[0; MAX_SIZE]; 3];
    for i in 0..n {
        let nums: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        a[0][i] = nums[0];
        a[1][i] = nums[1];
        a[2][i] = nums[2];
    }

    let result = compute_min_difference(n, &x, &a);
    println!("{}", result);
}
