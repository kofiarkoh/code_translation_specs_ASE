use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    a: i32,
    b: i32,
    d: i32,
    f: bool,
}

fn combi(
    k: usize,
    a: i32,
    w: i32,
    n: i32,
    m: usize,
    tbl: &mut [Edge],
    ans: &mut [i32; 12],
    tmp: &mut [i32; 12],
    len: &mut i32,
    num: &mut usize,
) {
    for b in 1..=n {
        if b == a {
            continue;
        }
        for i in 0..m {
            let e = tbl[i];
            if ((e.a == a && e.b == b) || (e.a == b && e.b == a)) && !e.f {
                tbl[i].f = true;
                tmp[k] = b;
                // C uses signed int; overflow is UB there, but typical judge inputs avoid it.
                // We use wrapping_add to avoid Rust panics and keep "machine int" behavior.
                let nw = w.wrapping_add(e.d);
                combi(k + 1, b, nw, n, m, tbl, ans, tmp, len, num);
                tbl[i].f = false;
            }
        }
    }

    if w > *len {
        *num = k;
        *len = w;
        // memcpy(ans, tmp, sizeof(int)*num)
        for i in 0..*num {
            ans[i] = tmp[i];
        }
    }
}

fn main() {
    // Read all input, parse as integers like scanf("%d")
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        let m: usize = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };
        if n == 0 {
            break;
        }

        let mut tbl = vec![
            Edge {
                a: 0,
                b: 0,
                d: 0,
                f: false
            };
            m
        ];
        for i in 0..m {
            let a: i32 = it.next().unwrap().parse().unwrap();
            let b: i32 = it.next().unwrap().parse().unwrap();
            let d: i32 = it.next().unwrap().parse().unwrap();
            tbl[i] = Edge { a, b, d, f: false };
        }

        let mut ans = [0i32; 12];
        let mut tmp = [0i32; 12];
        let mut len: i32 = 0;
        let mut num: usize = 0;

        for a in 1..=n {
            for b in 1..=n {
                if b == a {
                    continue;
                }
                for i in 0..m {
                    let e = tbl[i];
                    if ((e.a == a && e.b == b) || (e.b == a && e.a == b)) && !e.f {
                        tbl[i].f = true;
                        tmp[0] = a;
                        tmp[1] = b;
                        combi(
                            2,
                            b,
                            e.d,
                            n,
                            m,
                            &mut tbl,
                            &mut ans,
                            &mut tmp,
                            &mut len,
                            &mut num,
                        );
                        tbl[i].f = false;
                    }
                }
            }
        }

        out.push_str(&format!("{len}\n"));
        // Match C output: always prints ans[0] even if num==0 (then ans[0] is 0).
        out.push_str(&format!("{}", ans[0]));
        for i in 1..num {
            out.push_str(&format!(" {}", ans[i]));
        }
        out.push('\n');
    }

    print!("{out}");
}
