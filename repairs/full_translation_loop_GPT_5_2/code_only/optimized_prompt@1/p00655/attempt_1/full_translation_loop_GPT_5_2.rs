use std::io::{self, Read};

const FH: usize = 0; // First half
const SH: usize = 1; // Second half
const ALL: usize = 2;
const MAX: usize = 200005;

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
    fn in_i32(&mut self) -> i32 {
        // Emulate the C in(): skip until >= '0', then read while >= '0'.
        // Note: C code treats any char >= '0' as a digit and uses (c & 0xf).
        // We preserve that behavior.
        let n = self.buf.len();
        while self.idx < n && self.buf[self.idx] < b'0' {
            self.idx += 1;
        }
        let mut val: i32 = 0;
        while self.idx < n && self.buf[self.idx] >= b'0' {
            let c = self.buf[self.idx];
            val = val.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
            self.idx += 1;
        }
        val
    }
}

struct DS {
    que: [[i32; MAX]; 2],
    qt: [usize; 2],
    qe: [usize; 2],

    minq: [[i32; MAX]; 2],
    at: [usize; 2],
    ae: [usize; 2],

    maxq: [[i32; MAX]; 2],
    zt: [usize; 2],
    ze: [usize; 2],
}

impl DS {
    fn new() -> Self {
        Self {
            que: [[0; MAX]; 2],
            qt: [0; 2],
            qe: [0; 2],
            minq: [[0; MAX]; 2],
            at: [0; 2],
            ae: [0; 2],
            maxq: [[0; MAX]; 2],
            zt: [0; 2],
            ze: [0; 2],
        }
    }

    #[inline]
    fn size(&self, id: usize) -> usize {
        self.qe[id] - self.qt[id]
    }
    #[inline]
    fn min_sz(&self, id: usize) -> usize {
        self.ae[id] - self.at[id]
    }
    #[inline]
    fn max_sz(&self, id: usize) -> usize {
        self.ze[id] - self.zt[id]
    }

    fn add(&mut self, id: usize, a: i32) {
        if id == ALL {
            self.add(SH, a);
            if self.size(FH) < self.size(SH) {
                let x = self.del(SH);
                self.add(FH, x);
            }
            return;
        }

        self.que[id][self.qe[id]] = a;
        self.qe[id] += 1;

        if self.size(id) == 1 {
            self.minq[id][self.ae[id]] = a;
            self.ae[id] += 1;
            self.maxq[id][self.ze[id]] = a;
            self.ze[id] += 1;
        } else if id == FH {
            if self.minq[id][self.ae[id] - 1] > a {
                self.minq[id][self.ae[id]] = a;
                self.ae[id] += 1;
            }
            if self.maxq[id][self.ze[id] - 1] < a {
                self.maxq[id][self.ze[id]] = a;
                self.ze[id] += 1;
            }
        } else {
            while self.min_sz(id) > 0 && self.minq[id][self.ae[id] - 1] > a {
                self.ae[id] -= 1;
            }
            while self.max_sz(id) > 0 && self.maxq[id][self.ze[id] - 1] < a {
                self.ze[id] -= 1;
            }
            self.minq[id][self.ae[id]] = a;
            self.ae[id] += 1;
            self.maxq[id][self.ze[id]] = a;
            self.ze[id] += 1;
        }
    }

    fn del(&mut self, id: usize) -> i32 {
        let a: i32;
        if id == ALL {
            a = self.del(FH);
            if self.size(FH) < self.size(SH) {
                let x = self.del(SH);
                self.add(FH, x);
            }
        } else if id == FH {
            self.qe[id] -= 1;
            a = self.que[id][self.qe[id]];
            if self.minq[id][self.ae[id] - 1] == a {
                self.ae[id] -= 1;
            }
            if self.maxq[id][self.ze[id] - 1] == a {
                self.ze[id] -= 1;
            }
        } else {
            a = self.que[id][self.qt[id]];
            self.qt[id] += 1;
            if self.minq[id][self.at[id]] == a {
                self.at[id] += 1;
            }
            if self.maxq[id][self.zt[id]] == a {
                self.zt[id] += 1;
            }
        }
        a
    }

    fn gmin(&self, id: usize, th: usize) -> i32 {
        if id == FH {
            self.minq[id][self.ae[id] - th]
        } else {
            self.minq[id][self.at[id] + th - 1]
        }
    }

    fn gmax(&self, id: usize, th: usize) -> i32 {
        if id == FH {
            self.maxq[id][self.ze[id] - th]
        } else {
            self.maxq[id][self.zt[id] + th - 1]
        }
    }

    fn reset(&mut self) {
        self.qt = [0, 0];
        self.qe = [0, 0];
        self.at = [0, 0];
        self.ae = [0, 0];
        self.zt = [0, 0];
        self.ze = [0, 0];
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut ds = DS::new();
    let mut out = String::new();

    loop {
        let q = sc.in_i32();
        if q == 0 {
            break;
        }
        ds.reset();

        let mut qq = q;
        while qq > 0 {
            qq -= 1;
            let op = sc.in_i32();
            let a: i32;
            if op == 0 {
                let x = sc.in_i32();
                ds.add(ALL, x);
                continue;
            } else if op == 1 {
                a = ds.del(ALL);
            } else if op == 2 {
                a = ds.gmin(FH, 1);
            } else if op == 3 {
                a = ds.gmin(SH, 1);
            } else if op == 4 {
                let th = sc.in_i32() as usize;
                a = ds.gmin(FH, th);
            } else if op == 5 {
                let th = sc.in_i32() as usize;
                a = ds.gmin(SH, th);
            } else if op == 6 {
                a = ds.gmax(FH, 1);
            } else if op == 7 {
                a = ds.gmax(SH, 1);
            } else if op == 8 {
                let th = sc.in_i32() as usize;
                a = ds.gmax(FH, th);
            } else {
                let th = sc.in_i32() as usize;
                a = ds.gmax(SH, th);
            }
            out.push_str(&format!("{a}\n"));
        }
        out.push_str("end\n");
    }

    print!("{out}");
}
