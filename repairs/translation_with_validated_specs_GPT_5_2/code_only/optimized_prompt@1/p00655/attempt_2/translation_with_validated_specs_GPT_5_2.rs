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
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        Self { buf: s.into_bytes(), idx: 0 }
    }

    fn in_i32(&mut self) -> i32 {
        // Match the C code:
        // int n=0,c=getchar_unlocked();
        // while (c < '0') c = getchar_unlocked();
        // do n = 10*n + (c & 0xf), c = getchar_unlocked(); while (c >= '0');
        //
        // If input ends unexpectedly, behave like returning 0 (best-effort).
        let mut n: i32 = 0;

        while self.idx < self.buf.len() && self.buf[self.idx] < b'0' {
            self.idx += 1;
        }
        if self.idx >= self.buf.len() {
            return 0;
        }

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
            // Same as C: only push if it extends the monotone stack.
            if self.minq[id][self.ae[id] - 1] > a {
                self.minq[id][self.ae[id]] = a;
                self.ae[id] += 1;
            }
            if self.maxq[id][self.ze[id] - 1] < a {
                self.maxq[id][self.ze[id]] = a;
                self.ze[id] += 1;
            }
        } else {
            // SH: pop from back to maintain monotone queue.
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
            // C does: a = que[id][--qe[id]];
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

        // Reset indices (arrays left intact, like C).
        st.qt = [0, 0];
        st.qe = [0, 0];
        st.at = [0, 0];
        st.ae = [0, 0];
        st.zt = [0, 0];
        st.ze = [0, 0];

        for _ in 0..q {
            let op = sc.in_i32();
            let a: i32;

            if op == 0 {
                st.add(ALL, sc.in_i32());
                continue;
            } else if op == 1 {
                // In C, del(ALL) is called even if empty (UB). Avoid panicking:
                // if empty, return 0 (best-effort) to keep program running.
                if st.size(FH) + st.size(SH) == 0 {
                    a = 0;
                } else {
                    a = st.del(ALL);
                }
            } else if op == 2 {
                if st.min_sz(FH) == 0 {
                    a = 0;
                } else {
                    a = st.gmin(FH, 1);
                }
            } else if op == 3 {
                if st.min_sz(SH) == 0 {
                    a = 0;
                } else {
                    a = st.gmin(SH, 1);
                }
            } else if op == 4 {
                let th = sc.in_i32().max(0) as usize;
                if th == 0 || th > st.min_sz(FH) {
                    a = 0;
                } else {
                    a = st.gmin(FH, th);
                }
            } else if op == 5 {
                let th = sc.in_i32().max(0) as usize;
                if th == 0 || th > st.min_sz(SH) {
                    a = 0;
                } else {
                    a = st.gmin(SH, th);
                }
            } else if op == 6 {
                if st.max_sz(FH) == 0 {
                    a = 0;
                } else {
                    a = st.gmax(FH, 1);
                }
            } else if op == 7 {
                if st.max_sz(SH) == 0 {
                    a = 0;
                } else {
                    a = st.gmax(SH, 1);
                }
            } else if op == 8 {
                let th = sc.in_i32().max(0) as usize;
                if th == 0 || th > st.max_sz(FH) {
                    a = 0;
                } else {
                    a = st.gmax(FH, th);
                }
            } else {
                let th = sc.in_i32().max(0) as usize;
                if th == 0 || th > st.max_sz(SH) {
                    a = 0;
                } else {
                    a = st.gmax(SH, th);
                }
            }

            out.push_str(&format!("{a}\n"));
        }

        out.push_str("end\n");
    }

    print!("{out}");
}
