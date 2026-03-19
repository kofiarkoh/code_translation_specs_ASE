use std::io::{self, Read};

const MAX: usize = 100_002;
const MOD: i32 = 10007;

struct DSU {
    id: Vec<usize>,
    size: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut id = vec![0usize; n];
        let mut size = vec![1i32; n];
        for i in 0..n {
            id[i] = i;
            size[i] = 1;
        }
        Self { id, size }
    }

    fn root(&mut self, mut i: usize) -> usize {
        while i != self.id[i] {
            // Path halving: id[i] = id[id[i]]
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

// Emulates the C in() which assumes non-negative integers and reads until a non-digit (< '0').
// It does not skip leading whitespace; it will treat any non-digit as terminator and return 0
// if called at EOF or if the first byte is non-digit.
struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        Self { buf: input, idx: 0 }
    }

    fn getchar_unlocked(&mut self) -> i32 {
        if self.idx >= self.buf.len() {
            return -1; // EOF
        }
        let c = self.buf[self.idx] as i32;
        self.idx += 1;
        c
    }

    fn in_int(&mut self) -> i32 {
        let mut n: i32 = 0;
        let mut c = self.getchar_unlocked();
        // do-while: always executes once, even if c is non-digit or EOF
        loop {
            n = (n << 3) + (n << 1) + (c & 0x0f);
            c = self.getchar_unlocked();
            if c < ('0' as i32) {
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

    // Preallocate fixed-size arrays like the C code.
    let mut f = vec![[0i32; 2]; MAX];
    let mut maxv = vec![0i32; MAX];
    let mut cnt = vec![0i32; MAX];

    let mut out = String::new();

    loop {
        let n_i32 = sc.in_int();
        if n_i32 == 0 {
            break;
        }
        let n = n_i32 as usize;

        let mut dsu = DSU::new(n);

        for i in 0..n {
            let a = sc.in_int() as usize;
            f[i][0] = sc.in_int();
            let b = sc.in_int() as usize;
            f[i][1] = sc.in_int();
            dsu.unite(i, a);
            dsu.unite(i, b);
        }

        // memset(cnt, 0, sizeof(cnt));
        for x in cnt.iter_mut() {
            *x = 0;
        }

        for i in 0..n {
            let k = dsu.root(i);
            check(k, f[i][0], &mut maxv, &mut cnt);
            check(k, f[i][1], &mut maxv, &mut cnt);
        }

        let mut ans: i32 = 1;
        for i in 0..n {
            if cnt[i] != 0 {
                ans = (ans * (cnt[i] >> 1)) % MOD;
            }
        }
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
