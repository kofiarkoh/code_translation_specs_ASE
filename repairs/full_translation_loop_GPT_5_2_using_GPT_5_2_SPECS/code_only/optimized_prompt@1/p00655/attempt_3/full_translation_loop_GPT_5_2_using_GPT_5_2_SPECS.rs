use std::io::{self, Read};

const FH: usize = 0;
const SH: usize = 1;
const ALL: usize = 2;
const MAX: usize = 200005;

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
    fn in_i32(&mut self) -> i32 {
        // Same behavior as the C in(): skip while c < '0', then read while c >= '0'.
        // If input is exhausted, return 0 (C would loop on EOF; AOJ inputs are well-formed,
        // but this prevents infinite loops/timeouts on unexpected EOF).
        while self.idx < self.buf.len() && self.buf[self.idx] < b'0' {
            self.idx += 1;
        }
        if self.idx >= self.buf.len() {
            return 0;
        }
        let mut n: i32 = 0;
        while self.idx < self.buf.len() && self.buf[self.idx] >= b'0' {
            let c = self.buf[self.idx];
            n = n.wrapping_mul(10).wrapping_add((c & 0x0f) as i32);
            self.idx += 1;
        }
        n
    }
}

struct State {
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

impl State {
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
            // C would do unchecked reads here; we guard to avoid panics.
            if self.ae[id] > 0 && self.minq[id][self.ae[id] - 1] == a {
                self.ae[id] -= 1;
            }
            if self.ze[id] > 0 && self.maxq[id][self.ze[id] - 1] == a {
                self.ze[id] -= 1;
            }
        } else {
            a = self.que[id][self.qt[id]];
            self.qt[id] += 1;
            if self.at[id] < self.ae[id] && self.minq[id][self.at[id]] == a {
                self.at[id] += 1;
            }
            if self.zt[id] < self.ze[id] && self.maxq[id][self.zt[id]] == a {
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
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();
    let mut st = State::new();

    loop {
        let q = sc.in_i32();
        if q == 0 {
            break;
        }

        st.qt = [0, 0];
        st.qe = [0, 0];
        st.at = [0, 0];
        st.ae = [0, 0];
        st.zt = [0, 0];
        st.ze = [0, 0];

        for _ in 0..q {
            let op = sc.in_i32();
            if op == 0 {
                let v = sc.in_i32();
                st.add(ALL, v);
                continue;
            }
            let a = match op {
                1 => st.del(ALL),
                2 => st.gmin(FH, 1),
                3 => st.gmin(SH, 1),
                4 => st.gmin(FH, sc.in_i32() as usize),
                5 => st.gmin(SH, sc.in_i32() as usize),
                6 => st.gmax(FH, 1),
                7 => st.gmax(SH, 1),
                8 => st.gmax(FH, sc.in_i32() as usize),
                _ => st.gmax(SH, sc.in_i32() as usize),
            };
            out.push_str(&a.to_string());
            out.push('\n');
        }
        out.push_str("end\n");
    }

    print!("{out}");
}
