use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as integers like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let mut s = vec![0i32; n];
    let mut t = vec![0i32; n];
    let mut x = vec![0i32; n];
    for i in 0..n {
        x[i] = it.next().unwrap().parse().unwrap();
    }

    let mut a = vec![vec![0i32; n], vec![0i32; n], vec![0i32; n]];
    for i in 0..n {
        a[0][i] = it.next().unwrap().parse().unwrap();
        a[1][i] = it.next().unwrap().parse().unwrap();
        a[2][i] = it.next().unwrap().parse().unwrap();
    }

    // f(p): for each i, count how many p[j] > p[i], then t[i] = x[cnt]
    // Assumes cnt is within 0..n (as in the C code; otherwise C would be UB).
    let mut f = |p: &Vec<i32>, t: &mut Vec<i32>, x: &Vec<i32>| {
        let n = p.len();
        for i in 0..n {
            let mut cnt: usize = 0;
            for j in 0..n {
                if p[j] > p[i] {
                    cnt += 1;
                }
            }
            t[i] = x[cnt];
        }
    };

    let mut ans: i32 = 1000;

    for k in 0..3usize {
        for i in 0..n {
            s[i] = 0;
        }

        for kk in 0..3usize {
            if kk != k {
                f(&a[kk], &mut t, &x);
                for i in 0..n {
                    s[i] += t[i];
                }
            }
        }

        let ori = a[k][0];
        for i in 1..n {
            for d in 0..2i32 {
                if a[k][i] + d > ori {
                    a[k][0] = a[k][i] + d;
                    f(&a[k], &mut t, &x);

                    let mut cnt: i32 = 0;
                    let base = s[0] + t[0];
                    for j in 0..n {
                        if s[j] + t[j] > base {
                            cnt += 1;
                        }
                    }
                    if cnt <= 7 {
                        let diff = a[k][0] - ori;
                        if diff < ans {
                            ans = diff;
                        }
                    }
                }
            }
        }
        a[k][0] = ori;
    }

    if ans == 1000 {
        print!("Saiko\n");
    } else {
        print!("{}\n", ans);
    }
}
