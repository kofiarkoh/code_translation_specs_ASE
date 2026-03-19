use std::io::{self, Read};

const HASHSIZ: usize = 1_000_003;
const INF: i32 = 0x7fffffff;

#[derive(Clone, Copy)]
struct HA {
    n: i32,
    b: i32,
}

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

    // Emulates the C `in()`:
    // - reads a non-negative integer
    // - assumes input is well-formed (digits then a non-digit)
    // - does not skip leading whitespace explicitly; it starts from current byte
    fn in_i32(&mut self) -> i32 {
        let mut n: i32 = 0;
        if self.idx >= self.buf.len() {
            return 0;
        }
        let mut c = self.buf[self.idx];
        self.idx += 1;

        loop {
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
            if self.idx >= self.buf.len() {
                break;
            }
            c = self.buf[self.idx];
            self.idx += 1;
            if c < b'0' {
                break;
            }
        }
        n
    }
}

fn abs_i32(x: i32) -> i32 {
    // C's ABS macro would overflow on INT_MIN (UB). Inputs here are expected to avoid that.
    if x >= 0 { x } else { -x }
}

fn lookup1(ha1: &[i32], n: i32) -> bool {
    let mut p = (n.rem_euclid(HASHSIZ as i32)) as usize;
    loop {
        let v = ha1[p];
        if v == 0 {
            return false;
        }
        if v == n {
            return true;
        }
        p += 1;
        if p == HASHSIZ {
            p = 0;
        }
    }
}

fn insert1(ha1: &mut [i32], n: i32) {
    let mut p = (n.rem_euclid(HASHSIZ as i32)) as usize;
    loop {
        let v = ha1[p];
        if v == 0 {
            ha1[p] = n;
            return;
        }
        if v == n {
            return;
        }
        p += 1;
        if p == HASHSIZ {
            p = 0;
        }
    }
}

fn lookup2(ha2: &mut [HA], n: i32, b: i32) -> bool {
    let mut p = (n.rem_euclid(HASHSIZ as i32)) as usize;
    loop {
        let cur_n = ha2[p].n;
        if cur_n == 0 {
            return false;
        }
        if cur_n == n {
            if ha2[p].b == b - 1 {
                ha2[p].b = b;
                return true;
            }
            return false;
        }
        p += 1;
        if p == HASHSIZ {
            p = 0;
        }
    }
}

fn insert2(ha2: &mut [HA], n: i32) {
    let mut p = (n.rem_euclid(HASHSIZ as i32)) as usize;
    loop {
        let cur_n = ha2[p].n;
        if cur_n == 0 {
            ha2[p].n = n;
            ha2[p].b = 1;
            return;
        }
        if cur_n == n {
            return;
        }
        p += 1;
        if p == HASHSIZ {
            p = 0;
        }
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    // Hash tables (0 means empty, matching C's memset-to-zero sentinel usage)
    let mut ha1 = vec![0i32; HASHSIZ + 5];
    let mut ha2 = vec![HA { n: 0, b: 0 }; HASHSIZ + 5];

    // Fixed-size arrays as in C
    let mut q = vec![0i32; 60000]; // 3^10
    let mut a = vec![0i32; 102];
    let mut w = vec![0i32; 12];

    loop {
        let n = sc.in_i32();
        if n == 0 {
            break;
        }
        let m = sc.in_i32();

        for i in 0..(n as usize) {
            a[i] = sc.in_i32();
        }
        for i in 0..(m as usize) {
            w[i] = sc.in_i32();
        }

        q[0] = 0;
        let mut end: usize = 1;
        for i in 0..(m as usize) {
            let e = end;
            for j in 0..e {
                let base = q[j];
                q[end] = base.wrapping_add(w[i]);
                end += 1;
                q[end] = base.wrapping_sub(w[i]);
                end += 1;
            }
        }

        // memset(ha1, 0, sizeof(ha1));
        for v in ha1.iter_mut() {
            *v = 0;
        }
        for i in 0..end {
            if q[i] > 0 {
                insert1(&mut ha1, q[i]);
            }
        }

        let mut x: i32 = 0;
        let mut ans: i32 = 0;

        for i in 0..(n as usize) {
            if !lookup1(&ha1, a[i]) {
                if x == 0 {
                    ans = INF;
                    // memset(ha2, 0, sizeof(ha2));
                    for v in ha2.iter_mut() {
                        *v = HA { n: 0, b: 0 };
                    }
                    for j in 0..end {
                        let t = abs_i32(a[i].wrapping_sub(q[j]));
                        if t < ans {
                            ans = t;
                        }
                        insert2(&mut ha2, t);
                    }
                    x = 1;
                } else {
                    x += 1;
                    ans = INF;
                    let mut r = 0;
                    for j in 0..end {
                        let t = abs_i32(a[i].wrapping_sub(q[j]));
                        if lookup2(&mut ha2, t, x) {
                            if t < ans {
                                ans = t;
                            }
                            r = 1;
                        }
                    }
                    if r == 0 {
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
