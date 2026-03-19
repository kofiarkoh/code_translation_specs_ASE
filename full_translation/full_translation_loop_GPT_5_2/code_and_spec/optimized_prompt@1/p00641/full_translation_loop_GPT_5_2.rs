use std::io::{self, Read};

const MAX: usize = 100_002;
const MOD: i32 = 10007;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            size.push(1);
        }
        Self { parent, size }
    }

    fn root(&mut self, mut i: usize) -> usize {
        while i != self.parent[i] {
            // Path compression by halving (matches C behavior closely)
            let p = self.parent[i];
            self.parent[i] = self.parent[p];
            i = self.parent[i];
        }
        i
    }

    fn unite(&mut self, p: usize, q: usize) {
        let mut i = self.root(p);
        let mut j = self.root(q);
        if i == j {
            return;
        }
        if self.size[i] < self.size[j] {
            std::mem::swap(&mut i, &mut j);
        }
        // i is larger
        self.parent[j] = i;
        self.size[i] += self.size[j];
    }
}

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self {
            buf: input.into_bytes(),
            idx: 0,
        }
    }

    fn next_usize(&mut self) -> Option<usize> {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        if self.idx >= self.buf.len() {
            return None;
        }
        let mut n: usize = 0;
        while self.idx < self.buf.len() && !self.buf[self.idx].is_ascii_whitespace() {
            let c = self.buf[self.idx];
            // The original C `in()` assumes digits; we preserve that assumption.
            n = n * 10 + (c - b'0') as usize;
            self.idx += 1;
        }
        Some(n)
    }
}

fn check(k: usize, fval: i32, maxv: &mut [i32], cnt: &mut [i32]) {
    if cnt[k] == 0 || fval > maxv[k] {
        cnt[k] = 1;
        maxv[k] = fval;
    } else if fval == maxv[k] {
        cnt[k] += 1;
    }
}

fn main() {
    let mut sc = Scanner::new();

    let mut out = String::new();

    loop {
        let n = match sc.next_usize() {
            Some(v) => v,
            None => break,
        };
        if n == 0 {
            break; // matches `while (n = in())`
        }
        // Spec says n must not exceed maximum limit; C uses fixed MAX arrays.
        // We keep the same effective constraint.
        assert!(n <= MAX);

        let mut uf = UnionFind::new(n);
        let mut f0: Vec<i32> = vec![0; n];
        let mut f1: Vec<i32> = vec![0; n];

        for i in 0..n {
            let a = sc.next_usize().unwrap();
            let fv0 = sc.next_usize().unwrap() as i32;
            let b = sc.next_usize().unwrap();
            let fv1 = sc.next_usize().unwrap() as i32;

            f0[i] = fv0;
            f1[i] = fv1;

            uf.unite(i, a);
            uf.unite(i, b);
        }

        let mut maxv: Vec<i32> = vec![0; n];
        let mut cnt: Vec<i32> = vec![0; n];

        for i in 0..n {
            let k = uf.root(i);
            check(k, f0[i], &mut maxv, &mut cnt);
            check(k, f1[i], &mut maxv, &mut cnt);
        }

        let mut ans: i32 = 1;
        for i in 0..n {
            if cnt[i] != 0 {
                // C uses (cnt[i] >> 1) i.e., floor(cnt/2)
                ans = (ans * (cnt[i] >> 1)) % MOD;
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
