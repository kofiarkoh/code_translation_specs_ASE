use std::io::{self, Read};

fn max_i32(a: i32, b: i32) -> i32 {
    if a < b { b } else { a }
}

struct Solver {
    // Large fixed-size buffers as in the C code.
    q: Vec<i32>,      // [1000000]
    cdist: Vec<i32>,  // C[100010]
    ta: Vec<i32>,     // ta[100010]
    nt: Vec<i32>,     // nt[200010]
    fr: Vec<i32>,     // fr[200010]
    to: Vec<i32>,     // to[200010]
    co: Vec<i32>,     // co[200010]
    f: Vec<i32>,      // f[100010] used as boolean 0/1
}

impl Solver {
    fn new() -> Self {
        Self {
            q: vec![0; 1_000_000],
            cdist: vec![0; 100_010],
            ta: vec![0; 100_010],
            nt: vec![0; 200_010],
            fr: vec![0; 200_010],
            to: vec![0; 200_010],
            co: vec![0; 200_010],
            f: vec![0; 100_010],
        }
    }

    fn dfs(&mut self, _v: i32, _e: i32, s: i32, d: i32) -> i32 {
        let mut maxv = d;
        let mut i = self.ta[s as usize];
        // C loop: for(i=ta[s]; i+1; i=nt[i])
        while i + 1 != 0 {
            let ti = self.to[i as usize] as usize;
            if self.f[ti] != 0 {
                i = self.nt[i as usize];
                continue;
            }
            self.f[ti] = 1;
            // Preserve C's signed overflow behavior as wrapping.
            let ns = s as usize;
            self.cdist[ti] = self.cdist[ns].wrapping_add(self.co[i as usize]);
            let rec = self.dfs(_v, _e, self.to[i as usize], d.wrapping_add(self.co[i as usize]));
            maxv = max_i32(maxv, rec);
            i = self.nt[i as usize];
        }
        maxv
    }

    #[allow(dead_code)]
    fn bfs(&mut self, v: i32, _e: i32, s: i32, g: &mut i32) -> i32 {
        let mut maxv: i32 = 0;
        for i in 0..(v as usize) {
            self.cdist[i] = 1_000_000_000;
        }
        self.q[0] = s;
        self.cdist[s as usize] = 0;
        let mut t: i32 = 0;
        let mut r: i32 = 1;

        // C loop: for(r=1; r-t; t++)
        while r - t != 0 {
            let qt = self.q[t as usize] as usize;
            let mut i = self.ta[qt];
            while i + 1 != 0 {
                let fr = self.fr[i as usize] as usize;
                let to = self.to[i as usize] as usize;
                let cand = self.cdist[fr].wrapping_add(self.co[i as usize]);
                if self.cdist[to] > cand {
                    self.cdist[to] = cand;
                    if maxv < self.cdist[to] {
                        maxv = self.cdist[to];
                        *g = self.to[i as usize];
                    }
                    self.q[r as usize] = self.to[i as usize];
                    r += 1;
                }
                i = self.nt[i as usize];
            }
            t += 1;
        }
        maxv
    }

    fn tyokkei(&mut self, v: i32, e: i32, a: &[i32], b: &[i32], c: &[i32]) -> i32 {
        for i in 0..(v as usize) {
            self.ta[i] = -1;
            self.cdist[i] = 1_000_000_000;
            self.f[i] = 0;
        }
        self.cdist[0] = 0;
        self.f[0] = 1;

        for i in 0..(e as usize) {
            // forward edge i
            let ai = a[i] as usize;
            self.nt[i] = self.ta[ai];
            self.fr[i] = a[i];
            self.to[i] = b[i];
            self.co[i] = c[i];
            self.ta[ai] = i as i32;

            // reverse edge i+e
            let j = i + e as usize;
            let bi = b[i] as usize;
            self.nt[j] = self.ta[bi];
            self.fr[j] = b[i];
            self.to[j] = a[i];
            self.co[j] = c[i];
            self.ta[bi] = j as i32;
        }

        let max1 = self.dfs(v, e * 2, 0, 0);

        let mut g: i32 = 0;
        for i in 0..(v as usize) {
            if max1 == self.cdist[i] {
                g = i as i32;
            }
            self.cdist[i] = 1_000_000_000;
            self.f[i] = 0;
        }
        self.cdist[g as usize] = 0;
        self.f[g as usize] = 1;

        self.dfs(v, e * 2, g, 0)
    }
}

fn main() {
    // Read all input (scanf-like whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut a = vec![0i32; 100_010];
    let mut b = vec![0i32; 100_010];
    let mut c = vec![0i32; 100_010];

    for i in 0..((n - 1) as usize) {
        a[i] = it.next().unwrap().parse().unwrap();
        b[i] = it.next().unwrap().parse().unwrap();
        c[i] = it.next().unwrap().parse().unwrap();
    }

    let mut solver = Solver::new();
    let ans = solver.tyokkei(n, n - 1, &a, &b, &c);
    print!("{}\n", ans);
}
