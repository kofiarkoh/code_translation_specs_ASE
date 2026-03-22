use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    a: i32,
    b: i32,
    d: i32,
    f: bool,
}

struct Solver {
    tbl: Vec<Edge>,
    n: i32,
    m: usize,
    ans: [i32; 12],
    tmp: [i32; 12],
    len: i32,
    num: usize,
}

impl Solver {
    fn combi(&mut self, k: usize, a: i32, w: i32) {
        for b in 1..=self.n {
            if b == a {
                continue;
            }
            for i in 0..self.m {
                let e = self.tbl[i];
                let connected = (e.a == a && e.b == b) || (e.a == b && e.b == a);
                if connected && !e.f {
                    self.tbl[i].f = true;
                    self.tmp[k] = b;
                    // C uses signed int; Rust i32 matches typical behavior.
                    // No special overflow emulation is added; inputs are expected to fit.
                    self.combi(k + 1, b, w + e.d);
                    self.tbl[i].f = false;
                }
            }
        }
        if w > self.len {
            self.num = k;
            self.len = w;
            // memcpy(ans, tmp, sizeof(int)*num)
            for i in 0..self.num {
                self.ans[i] = self.tmp[i];
            }
        }
    }
}

fn main() {
    // Read all input, then parse as integers like scanf("%d").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i32 = n_opt.unwrap().parse().unwrap();
        let m: i32 = match it.next() {
            Some(x) => x.parse().unwrap(),
            None => break,
        };
        if n == 0 {
            break;
        }

        let m_usize = m as usize;
        let mut tbl = Vec::with_capacity(m_usize);
        for _ in 0..m_usize {
            let a: i32 = it.next().unwrap().parse().unwrap();
            let b: i32 = it.next().unwrap().parse().unwrap();
            let d: i32 = it.next().unwrap().parse().unwrap();
            tbl.push(Edge { a, b, d, f: false });
        }

        let mut solver = Solver {
            tbl,
            n,
            m: m_usize,
            ans: [0; 12],
            tmp: [0; 12],
            len: 0,
            num: 0,
        };

        for a in 1..=solver.n {
            for b in 1..=solver.n {
                if b == a {
                    continue;
                }
                for i in 0..solver.m {
                    let e = solver.tbl[i];
                    let connected = (e.a == a && e.b == b) || (e.b == a && e.a == b);
                    if connected && !e.f {
                        solver.tbl[i].f = true;
                        solver.tmp[0] = a;
                        solver.tmp[1] = b;
                        solver.combi(2, b, e.d);
                        solver.tbl[i].f = false;
                    }
                }
            }
        }

        out.push_str(&format!("{}\n", solver.len));
        // Match C output: always prints ans[0] even if num==0 (would be uninitialized in C).
        // Here ans is zero-initialized; this is the closest safe approximation.
        out.push_str(&format!("{}", solver.ans[0]));
        for i in 1..solver.num {
            out.push_str(&format!(" {}", solver.ans[i]));
        }
        out.push('\n');
    }

    print!("{out}");
}
