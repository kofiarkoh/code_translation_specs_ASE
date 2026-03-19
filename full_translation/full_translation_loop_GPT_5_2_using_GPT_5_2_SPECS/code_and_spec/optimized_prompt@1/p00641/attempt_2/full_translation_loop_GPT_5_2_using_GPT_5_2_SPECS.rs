use std::io::{self, Read};

const MOD: i32 = 10007;

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            size.push(1);
        }
        Self { parent, size }
    }

    #[inline]
    fn root(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.parent[p];
            x = self.parent[x];
        }
        x
    }

    #[inline]
    fn unite(&mut self, a: usize, b: usize) {
        let mut ra = self.root(a);
        let mut rb = self.root(b);
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }
}

#[inline]
fn check(comp: usize, val: i32, maxv: &mut [i32], cnt: &mut [i32]) {
    if cnt[comp] == 0 || val > maxv[comp] {
        cnt[comp] = 1;
        maxv[comp] = val;
    } else if val == maxv[comp] {
        cnt[comp] += 1;
    }
}

/// Fast scanner for non-negative integers (matches the C `in()` behavior).
struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf).unwrap();
        Self { buf, idx: 0 }
    }

    #[inline]
    fn next_usize(&mut self) -> Option<usize> {
        let n = self.buf.len();
        while self.idx < n && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        if self.idx >= n {
            return None;
        }
        let mut val: usize = 0;
        while self.idx < n {
            let c = self.buf[self.idx];
            if !c.is_ascii_digit() {
                break;
            }
            val = val * 10 + (c - b'0') as usize;
            self.idx += 1;
        }
        Some(val)
    }

    #[inline]
    fn next_i32(&mut self) -> Option<i32> {
        self.next_usize().map(|v| v as i32)
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    while let Some(n) = sc.next_usize() {
        if n == 0 {
            break;
        }

        let mut dsu = DSU::new(n);
        let mut f0 = vec![0i32; n];
        let mut f1 = vec![0i32; n];

        for i in 0..n {
            let a = sc.next_usize().unwrap();
            let v0 = sc.next_i32().unwrap();
            let b = sc.next_usize().unwrap();
            let v1 = sc.next_i32().unwrap();

            f0[i] = v0;
            f1[i] = v1;

            dsu.unite(i, a);
            dsu.unite(i, b);
        }

        let mut maxv = vec![0i32; n];
        let mut cnt = vec![0i32; n];

        for i in 0..n {
            let k = dsu.root(i);
            check(k, f0[i], &mut maxv, &mut cnt);
            check(k, f1[i], &mut maxv, &mut cnt);
        }

        let mut ans: i32 = 1;
        for i in 0..n {
            if cnt[i] != 0 {
                ans = (ans * (cnt[i] / 2)) % MOD;
            }
        }

        out.push_str(ans.to_string().as_str());
        out.push('\n');
    }

    print!("{out}");
}
