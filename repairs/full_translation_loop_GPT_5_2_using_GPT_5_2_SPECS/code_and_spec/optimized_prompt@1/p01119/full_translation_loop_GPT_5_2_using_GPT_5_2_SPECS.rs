use std::io::{self, Read};

const INF: i32 = i32::MAX;
const HASHSIZ: usize = 1_000_003;

#[derive(Clone, Copy)]
struct HA {
    n: i32,
    b: i32,
}

fn abs_i32(x: i32) -> i32 {
    if x >= 0 { x } else { -x }
}

// Hash set for positive achievable sums (like ha1 in C).
// Uses open addressing with 0 as empty sentinel; values inserted are > 0.
struct Hash1 {
    tab: Vec<i32>,
}
impl Hash1 {
    fn new() -> Self {
        Self { tab: vec![0; HASHSIZ] }
    }
    fn clear(&mut self) {
        self.tab.fill(0);
    }
    fn lookup(&self, n: i32) -> bool {
        let mut idx = (n as usize) % HASHSIZ;
        loop {
            let v = self.tab[idx];
            if v == 0 {
                return false;
            }
            if v == n {
                return true;
            }
            idx += 1;
            if idx == HASHSIZ {
                idx = 0;
            }
        }
    }
    fn insert(&mut self, n: i32) {
        let mut idx = (n as usize) % HASHSIZ;
        loop {
            let v = self.tab[idx];
            if v == 0 {
                self.tab[idx] = n;
                return;
            }
            if v == n {
                return;
            }
            idx += 1;
            if idx == HASHSIZ {
                idx = 0;
            }
        }
    }
}

// Hash map from candidate adjustment t -> last-seen "b" counter (like ha2 in C).
// Uses open addressing with n==0 as empty sentinel; t can be 0, so store key as t+1.
struct Hash2 {
    tab: Vec<HA>,
}
impl Hash2 {
    fn new() -> Self {
        Self {
            tab: vec![HA { n: 0, b: 0 }; HASHSIZ],
        }
    }
    fn clear(&mut self) {
        self.tab.fill(HA { n: 0, b: 0 });
    }
    fn insert(&mut self, t: i32) {
        let key = t + 1; // ensure non-zero
        let mut idx = (key as usize) % HASHSIZ;
        loop {
            let e = self.tab[idx];
            if e.n == 0 {
                self.tab[idx] = HA { n: key, b: 1 };
                return;
            }
            if e.n == key {
                return;
            }
            idx += 1;
            if idx == HASHSIZ {
                idx = 0;
            }
        }
    }
    // Returns true iff key exists and its b == (b-1), then updates to b.
    fn lookup_update(&mut self, t: i32, b: i32) -> bool {
        let key = t + 1;
        let mut idx = (key as usize) % HASHSIZ;
        loop {
            let e = self.tab[idx];
            if e.n == 0 {
                return false;
            }
            if e.n == key {
                if e.b == b - 1 {
                    self.tab[idx].b = b;
                    return true;
                }
                return false;
            }
            idx += 1;
            if idx == HASHSIZ {
                idx = 0;
            }
        }
    }
}

fn main() {
    // Read all input, parse as non-negative integers (as per spec/C in()).
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let mut ha1 = Hash1::new();
    let mut ha2 = Hash2::new();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }
        let m: usize = it.next().unwrap().parse().unwrap();

        let mut a: Vec<i32> = Vec::with_capacity(n);
        for _ in 0..n {
            a.push(it.next().unwrap().parse::<i32>().unwrap());
        }
        let mut w: Vec<i32> = Vec::with_capacity(m);
        for _ in 0..m {
            w.push(it.next().unwrap().parse::<i32>().unwrap());
        }

        // Build all achievable signed sums q (including 0 and negatives).
        // Capacity implied by spec (<= 3^10) so Vec is fine.
        let mut q: Vec<i32> = Vec::with_capacity(60000);
        q.push(0);
        for &wi in &w {
            let e = q.len();
            for j in 0..e {
                let base = q[j];
                q.push(base + wi);
                q.push(base - wi);
            }
        }

        // Insert all positive achievable sums into ha1.
        ha1.clear();
        for &v in &q {
            if v > 0 {
                ha1.insert(v);
            }
        }

        // Determine answer.
        let mut x: i32 = 0; // number of unmeasurable targets processed so far
        let mut ans: i32 = 0;

        for &target in &a {
            // Per spec: 0 is considered measurable. C code also treats 0 as measurable
            // because ha1 stores only positive sums and lookup1(0) returns false,
            // but targets are non-negative; if target==0, it would be treated as
            // unmeasurable by C. This is a mismatch: spec says 0 measurable.
            // We follow the spec.
            if target == 0 {
                continue;
            }

            if !ha1.lookup(target) {
                if x == 0 {
                    ans = INF;
                    ha2.clear();
                    for &sum in &q {
                        let t = abs_i32(target - sum);
                        if t < ans {
                            ans = t;
                        }
                        ha2.insert(t);
                    }
                    x = 1;
                } else {
                    x += 1;
                    ans = INF;
                    let mut any = false;
                    for &sum in &q {
                        let t = abs_i32(target - sum);
                        if ha2.lookup_update(t, x) {
                            if t < ans {
                                ans = t;
                            }
                            any = true;
                        }
                    }
                    if !any {
                        ans = -1;
                        break;
                    }
                }
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
