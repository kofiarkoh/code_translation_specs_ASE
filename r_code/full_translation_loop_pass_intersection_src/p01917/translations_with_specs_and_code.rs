use std::io::{self, Read};

fn compute_scores(p: &[i32], x: &[i32]) -> Vec<i32> {
    let n = p.len();
    let mut t = vec![0i32; n];
    for i in 0..n {
        let mut cnt = 0usize;
        for j in 0..n {
            if p[j] > p[i] {
                cnt += 1;
            }
        }
        t[i] = x[cnt];
    }
    t
}

fn main() {
    // Read all input as whitespace-separated integers (matches scanf behavior for valid input).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // undefined by spec if missing; just exit
    };

    let mut x = vec![0i32; n];
    for i in 0..n {
        x[i] = it.next().unwrap().parse().unwrap();
    }

    let mut a = vec![vec![0i32; n]; 3];
    for i in 0..n {
        a[0][i] = it.next().unwrap().parse().unwrap();
        a[1][i] = it.next().unwrap().parse().unwrap();
        a[2][i] = it.next().unwrap().parse().unwrap();
    }

    let mut ans: i32 = 1000;

    for k in 0..3 {
        // s = sum of scores from the two unmodified sequences
        let mut s = vec![0i32; n];
        for kk in 0..3 {
            if kk == k {
                continue;
            }
            let t = compute_scores(&a[kk], &x);
            for i in 0..n {
                s[i] += t[i];
            }
        }

        // Try modifying only a[k][0] to match some other value in that sequence or +1,
        // but only if it strictly increases from original.
        let ori = a[k][0];
        for i in 1..n {
            for d in 0..=1 {
                let candidate = a[k][i] + d;
                if candidate > ori {
                    a[k][0] = candidate;
                    let t = compute_scores(&a[k], &x);

                    let base = s[0] + t[0];
                    let mut cnt = 0i32;
                    for j in 0..n {
                        if s[j] + t[j] > base {
                            cnt += 1;
                        }
                    }
                    if cnt <= 7 {
                        let inc = candidate - ori;
                        if inc < ans {
                            ans = inc;
                        }
                    }
                }
            }
        }
        // Restore original first element (spec invariant).
        a[k][0] = ori;
    }

    if ans == 1000 {
        // Fixed message when no qualifying increase found.
        println!("Saiko");
    } else {
        println!("{}", ans);
    }
}
