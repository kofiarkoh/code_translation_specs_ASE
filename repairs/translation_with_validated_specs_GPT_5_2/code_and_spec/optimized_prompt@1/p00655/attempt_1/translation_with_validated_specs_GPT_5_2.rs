use std::io::{self, Read};

const FH: usize = 0; // First half
const SH: usize = 1; // Second half
const ALL: usize = 2;

struct Scanner {
    input: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        Self { input, idx: 0 }
    }
    fn next_i32(&mut self) -> i32 {
        // Matches the C in(): skips until a digit, then reads a non-negative integer.
        while self.idx < self.input.len() && self.input[self.idx] < b'0' {
            self.idx += 1;
        }
        let mut n: i32 = 0;
        while self.idx < self.input.len() {
            let c = self.input[self.idx];
            if c < b'0' {
                break;
            }
            n = n * 10 + ((c & 0x0f) as i32);
            self.idx += 1;
        }
        n
    }
}

#[derive(Clone)]
struct State {
    // que[id] with head qt[id] and tail qe[id]
    que: [Vec<i32>; 2],
    qt: [usize; 2],
    qe: [usize; 2],

    // min tracking with head at[id] and tail ae[id]
    minq: [Vec<i32>; 2],
    at: [usize; 2],
    ae: [usize; 2],

    // max tracking with head zt[id] and tail ze[id]
    maxq: [Vec<i32>; 2],
    zt: [usize; 2],
    ze: [usize; 2],
}

impl State {
    fn new(cap: usize) -> Self {
        Self {
            que: [Vec::with_capacity(cap), Vec::with_capacity(cap)],
            qt: [0, 0],
            qe: [0, 0],
            minq: [Vec::with_capacity(cap), Vec::with_capacity(cap)],
            at: [0, 0],
            ae: [0, 0],
            maxq: [Vec::with_capacity(cap), Vec::with_capacity(cap)],
            zt: [0, 0],
            ze: [0, 0],
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
                let moved = self.del(SH);
                self.add(FH, moved);
            }
            return;
        }

        // push into que[id] at qe[id]
        if self.qe[id] == self.que[id].len() {
            self.que[id].push(a);
        } else {
            self.que[id][self.qe[id]] = a;
        }
        self.qe[id] += 1;

        if self.size(id) == 1 {
            // initialize tracking structures for this half
            if self.ae[id] == self.minq[id].len() {
                self.minq[id].push(a);
            } else {
                self.minq[id][self.ae[id]] = a;
            }
            self.ae[id] += 1;

            if self.ze[id] == self.maxq[id].len() {
                self.maxq[id].push(a);
            } else {
                self.maxq[id][self.ze[id]] = a;
            }
            self.ze[id] += 1;
        } else if id == FH {
            // FH: prefix-extrema history
            let last_min = self.minq[id][self.ae[id] - 1];
            if last_min > a {
                if self.ae[id] == self.minq[id].len() {
                    self.minq[id].push(a);
                } else {
                    self.minq[id][self.ae[id]] = a;
                }
                self.ae[id] += 1;
            }
            let last_max = self.maxq[id][self.ze[id] - 1];
            if last_max < a {
                if self.ze[id] == self.maxq[id].len() {
                    self.maxq[id].push(a);
                } else {
                    self.maxq[id][self.ze[id]] = a;
                }
                self.ze[id] += 1;
            }
        } else {
            // SH: monotone-queue style structures
            while self.min_sz(id) > 0 && self.minq[id][self.ae[id] - 1] > a {
                self.ae[id] -= 1;
            }
            while self.max_sz(id) > 0 && self.maxq[id][self.ze[id] - 1] < a {
                self.ze[id] -= 1;
            }

            if self.ae[id] == self.minq[id].len() {
                self.minq[id].push(a);
            } else {
                self.minq[id][self.ae[id]] = a;
            }
            self.ae[id] += 1;

            if self.ze[id] == self.maxq[id].len() {
                self.maxq[id].push(a);
            } else {
                self.maxq[id][self.ze[id]] = a;
            }
            self.ze[id] += 1;
        }
    }

    fn del(&mut self, id: usize) -> i32 {
        if id == ALL {
            let a = self.del(FH);
            if self.size(FH) < self.size(SH) {
                let moved = self.del(SH);
                self.add(FH, moved);
            }
            return a;
        }

        if id == FH {
            self.qe[id] -= 1;
            let a = self.que[id][self.qe[id]];
            if self.minq[id][self.ae[id] - 1] == a {
                self.ae[id] -= 1;
            }
            if self.maxq[id][self.ze[id] - 1] == a {
                self.ze[id] -= 1;
            }
            a
        } else {
            let a = self.que[id][self.qt[id]];
            self.qt[id] += 1;
            if self.minq[id][self.at[id]] == a {
                self.at[id] += 1;
            }
            if self.maxq[id][self.zt[id]] == a {
                self.zt[id] += 1;
            }
            a
        }
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

    loop {
        let q = sc.next_i32() as i32;
        if q == 0 {
            break;
        }

        // Capacity: spec says bounded; C uses MAX=200005. Use that to match behavior.
        let mut st = State::new(200_005);

        for _ in 0..q {
            let op = sc.next_i32();
            let a: i32;
            match op {
                0 => {
                    let x = sc.next_i32();
                    st.add(ALL, x);
                    continue; // no output
                }
                1 => a = st.del(ALL),
                2 => a = st.gmin(FH, 1),
                3 => a = st.gmin(SH, 1),
                4 => {
                    let k = sc.next_i32() as usize;
                    a = st.gmin(FH, k);
                }
                5 => {
                    let k = sc.next_i32() as usize;
                    a = st.gmin(SH, k);
                }
                6 => a = st.gmax(FH, 1),
                7 => a = st.gmax(SH, 1),
                8 => {
                    let k = sc.next_i32() as usize;
                    a = st.gmax(FH, k);
                }
                _ => {
                    let k = sc.next_i32() as usize;
                    a = st.gmax(SH, k);
                }
            }
            out.push_str(&format!("{a}\n"));
        }
        out.push_str("end\n");
    }

    print!("{out}");
}
