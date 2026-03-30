use std::io::{self, BufRead};

fn f(p: &[i32], n: usize, x: &mut [i32]) {
    for i in 0..n {
        let cnt = p.iter().filter(|&&val| val > p[i]).count();
        x[i] = x[cnt];
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut x = vec![0; n];
    let mut a = vec![vec![0; n]; 3];

    for i in 0..n {
        x[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }
    
    for i in 0..n {
        let nums: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        a[0][i] = nums[0];
        a[1][i] = nums[1];
        a[2][i] = nums[2];
    }

    let mut ans = 1000;
    let mut s = vec![0; n];
    let mut t = vec![0; n];

    for k in 0..3 {
        s.fill(0); // Reset s to zero for each outer loop iteration
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
                    a[k][0] = a[k][i] + d;
                    f(&a[k], n, &mut t);
                    let cnt = s.iter().zip(t.iter()).filter(|(&s_val, &t_val)| s_val + t_val > s[0] + t[0]).count();
                    if cnt <= 7 {
                        ans = ans.min(a[k][0] - ori);
                    }
                }
            }
        }
        a[k][0] = ori; // Restore original value
    }

    if ans == 1000 {
        println!("Saiko");
    } else {
        println!("{}", ans);
    }
}
