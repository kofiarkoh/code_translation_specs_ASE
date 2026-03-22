use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse::<i32>().unwrap() as usize,
        None => return,
    };

    let mut s = vec![0i32; n];
    let mut t = vec![0i32; n];
    let mut x = vec![0i32; n];
    let mut a = vec![vec![0i32; n]; 3];

    for i in 0..n {
        x[i] = it.next().unwrap().parse::<i32>().unwrap();
    }
    for i in 0..n {
        for k in 0..3 {
            a[k][i] = it.next().unwrap().parse::<i32>().unwrap();
        }
    }

    // f(p): for each i, count how many p[j] > p[i], then t[i] = x[cnt]
    // Note: In the C code, x is indexed by cnt without bounds checks.
    // We assume inputs keep cnt within [0, n-1]. (Otherwise C would be UB.)
    let mut f = |p: &Vec<i32>, t: &mut [i32], x: &[i32]| {
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

    for k in 0..3 {
        for i in 0..n {
            s[i] = 0;
        }

        for kk in 0..3 {
            if kk != k {
                f(&a[kk], &mut t, &x);
                for i in 0..n {
                    // C uses int; keep i32 and allow wrapping? Here values are expected small.
                    s[i] = s[i].wrapping_add(t[i]);
                }
            }
        }

        let ori = a[k][0];
        for i in 1..n {
            for d in 0..2 {
                let cand = a[k][i].wrapping_add(d);
                if cand > ori {
                    a[k][0] = cand;
                    f(&a[k], &mut t, &x);

                    let base = s[0].wrapping_add(t[0]);
                    let mut cnt: i32 = 0;
                    for j in 0..n {
                        if s[j].wrapping_add(t[j]) > base {
                            cnt += 1;
                        }
                    }
                    if cnt <= 7 {
                        let diff = a[k][0].wrapping_sub(ori);
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
