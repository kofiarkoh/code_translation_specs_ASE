use std::io::{self, Read};

const MAX: usize = 100_002;
const MOD: i32 = 10007;

struct UnionFind {
    id: Vec<usize>,
    size: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut id = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            id.push(i);
            size.push(1);
        }
        Self { id, size }
    }

    fn root(&mut self, mut i: usize) -> usize {
        while i != self.id[i] {
            // Path compression by halving: id[i] = id[id[i]]
            let p = self.id[i];
            let gp = self.id[p];
            self.id[i] = gp;
            i = self.id[i];
        }
        i
    }

    fn unite(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);
        if i == j {
            return;
        }
        if self.size[i] < self.size[j] {
            self.id[i] = j;
            self.size[j] += self.size[i];
        } else {
            self.id[j] = i;
            self.size[i] += self.size[j];
        }
    }
}

// Emulates the C in() which assumes non-negative integers and reads digits,
// stopping when the next char is < '0'. It does not skip whitespace; it
// expects the stream to be positioned at a digit when called.
struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { buf: input.into_bytes(), idx: 0 }
    }

    fn getchar_unlocked(&mut self) -> u8 {
        if self.idx >= self.buf.len() {
            0 // EOF sentinel; matches typical behavior enough for this program.
        } else {
            let c = self.buf[self.idx];
            self.idx += 1;
            c
        }
    }

    fn in_int(&mut self) -> i32 {
        let mut n: i32 = 0;
        let mut c = self.getchar_unlocked();
        // C code does: do { ... } while (c >= '0');
        // It assumes first c is a digit; we preserve that behavior.
        loop {
            n = (n << 3) + (n << 1) + ((c & 0x0f) as i32);
            c = self.getchar_unlocked();
            if c < b'0' {
                break;
            }
        }
        n
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

    // Static-sized arrays in C; here we allocate once at MAX to match indexing.
    let mut f0 = vec![0i32; MAX];
    let mut f1 = vec![0i32; MAX];
    let mut maxv = vec![0i32; MAX];
    let mut cnt = vec![0i32; MAX];

    loop {
        let n = sc.in_int();
        if n == 0 {
            break;
        }
        let n_usize = n as usize;

        let mut uf = UnionFind::new(n_usize);

        for i in 0..n_usize {
            let a = sc.in_int() as usize;
            f0[i] = sc.in_int();
            let b = sc.in_int() as usize;
            f1[i] = sc.in_int();
            uf.unite(i, a);
            uf.unite(i, b);
        }

        // memset(cnt, 0, sizeof(cnt));
        // In C this clears the whole MAX array; we do the same.
        cnt.fill(0);

        for i in 0..n_usize {
            let k = uf.root(i);
            check(k, f0[i], &mut maxv, &mut cnt);
            check(k, f1[i], &mut maxv, &mut cnt);
        }

        let mut ans: i32 = 1;
        for i in 0..n_usize {
            if cnt[i] != 0 {
                ans = (ans * (cnt[i] >> 1)) % MOD;
            }
        }
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
