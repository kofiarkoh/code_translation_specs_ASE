use std::io::{self, Read};

const MAX: usize = 200_005;

struct Scanner {
    buf: Vec<u8>,
    i: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        Self { buf: input, i: 0 }
    }

    fn next_u32(&mut self) -> Option<u32> {
        while self.i < self.buf.len() && !self.buf[self.i].is_ascii_digit() {
            self.i += 1;
        }
        if self.i >= self.buf.len() {
            return None;
        }
        let mut x: u32 = 0;
        while self.i < self.buf.len() && self.buf[self.i].is_ascii_digit() {
            x = x * 10 + (self.buf[self.i] - b'0') as u32;
            self.i += 1;
        }
        Some(x)
    }
}

/*
Key semantic fix:
FH "record" sequences are defined by scanning FH left->right in its CURRENT order.
FH is modified not only by push_back/pop_back, but also by receiving an element moved
from the FRONT of SH to the BACK of FH. That operation is an append, so incremental
record update is fine.

However, FH also loses elements via pop_back (deletion). If the popped element was a
record, simply popping from rec_min/rec_max is NOT sufficient because earlier elements
may become new records once the suffix is removed. Therefore, after any pop_back we
rebuild FH's record sequences from scratch based on current FH contents.
This matches the spec's definition and avoids incorrect query results.
*/

struct FH {
    a: Vec<i32>,
    rec_min: Vec<i32>,
    rec_max: Vec<i32>,
}
impl FH {
    fn new() -> Self {
        Self {
            a: Vec::with_capacity(MAX),
            rec_min: Vec::with_capacity(MAX),
            rec_max: Vec::with_capacity(MAX),
        }
    }
    fn len(&self) -> usize {
        self.a.len()
    }

    fn rebuild_records(&mut self) {
        self.rec_min.clear();
        self.rec_max.clear();
        for &x in &self.a {
            if self.rec_min.is_empty() || x < *self.rec_min.last().unwrap() {
                self.rec_min.push(x);
            }
            if self.rec_max.is_empty() || x > *self.rec_max.last().unwrap() {
                self.rec_max.push(x);
            }
        }
    }

    fn push_back(&mut self, x: i32) {
        self.a.push(x);
        // Incremental update is correct for append.
        if self.rec_min.is_empty() || x < *self.rec_min.last().unwrap() {
            self.rec_min.push(x);
        }
        if self.rec_max.is_empty() || x > *self.rec_max.last().unwrap() {
            self.rec_max.push(x);
        }
    }

    fn pop_back(&mut self) -> i32 {
        let _ = self.a.pop().unwrap();
        // Must rebuild to preserve scan-based record semantics after deletion.
        self.rebuild_records();
        // Return removed element (we already popped it; need value).
        // To avoid a second pop, capture before popping:
        unreachable!()
    }

    fn pop_back_ret(&mut self) -> i32 {
        let x = self.a.pop().unwrap();
        self.rebuild_records();
        x
    }

    fn mink_from_end(&self, k1: usize) -> i32 {
        self.rec_min[self.rec_min.len() - k1]
    }
    fn maxk_from_end(&self, k1: usize) -> i32 {
        self.rec_max[self.rec_max.len() - k1]
    }
}

struct SH {
    a: Vec<i32>,
    head: usize,

    minq: Vec<i32>,
    min_head: usize,
    maxq: Vec<i32>,
    max_head: usize,
}
impl SH {
    fn new() -> Self {
        Self {
            a: Vec::with_capacity(MAX),
            head: 0,
            minq: Vec::with_capacity(MAX),
            min_head: 0,
            maxq: Vec::with_capacity(MAX),
            max_head: 0,
        }
    }
    fn len(&self) -> usize {
        self.a.len() - self.head
    }
    fn push_back(&mut self, x: i32) {
        self.a.push(x);

        while self.minq.len() > self.min_head {
            let last = *self.minq.last().unwrap();
            if last <= x {
                break;
            }
            self.minq.pop();
        }
        self.minq.push(x);

        while self.maxq.len() > self.max_head {
            let last = *self.maxq.last().unwrap();
            if last >= x {
                break;
            }
            self.maxq.pop();
        }
        self.maxq.push(x);
    }
    fn pop_front(&mut self) -> i32 {
        let x = self.a[self.head];
        self.head += 1;

        if self.min_head < self.minq.len() && self.minq[self.min_head] == x {
            self.min_head += 1;
        }
        if self.max_head < self.maxq.len() && self.maxq[self.max_head] == x {
            self.max_head += 1;
        }
        x
    }
    fn mink_from_front(&self, k1: usize) -> i32 {
        self.minq[self.min_head + (k1 - 1)]
    }
    fn maxk_from_front(&self, k1: usize) -> i32 {
        self.maxq[self.max_head + (k1 - 1)]
    }
}

fn rebalance(fh: &mut FH, sh: &mut SH) {
    if fh.len() < sh.len() {
        let x = sh.pop_front();
        fh.push_back(x);
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    loop {
        let Some(q) = sc.next_u32() else { break };
        if q == 0 {
            break;
        }

        let mut fh = FH::new();
        let mut sh = SH::new();

        for _ in 0..q {
            let op = match sc.next_u32() {
                Some(v) => v,
                None => {
                    // Undefined behavior per spec on insufficient input; stop to avoid panic.
                    print!("{out}");
                    return;
                }
            };

            match op {
                0 => {
                    let x = match sc.next_u32() {
                        Some(v) => v as i32,
                        None => {
                            print!("{out}");
                            return;
                        }
                    };
                    sh.push_back(x);
                    rebalance(&mut fh, &mut sh);
                }
                1 => {
                    let x = fh.pop_back_ret();
                    out.push_str(&format!("{x}\n"));
                    rebalance(&mut fh, &mut sh);
                }
                2 => out.push_str(&format!("{}\n", fh.mink_from_end(1))),
                3 => out.push_str(&format!("{}\n", sh.mink_from_front(1))),
                4 => {
                    let k = match sc.next_u32() {
                        Some(v) => v as usize,
                        None => {
                            print!("{out}");
                            return;
                        }
                    };
                    out.push_str(&format!("{}\n", fh.mink_from_end(k)));
                }
                5 => {
                    let k = match sc.next_u32() {
                        Some(v) => v as usize,
                        None => {
                            print!("{out}");
                            return;
                        }
                    };
                    out.push_str(&format!("{}\n", sh.mink_from_front(k)));
                }
                6 => out.push_str(&format!("{}\n", fh.maxk_from_end(1))),
                7 => out.push_str(&format!("{}\n", sh.maxk_from_front(1))),
                8 => {
                    let k = match sc.next_u32() {
                        Some(v) => v as usize,
                        None => {
                            print!("{out}");
                            return;
                        }
                    };
                    out.push_str(&format!("{}\n", fh.maxk_from_end(k)));
                }
                9 => {
                    let k = match sc.next_u32() {
                        Some(v) => v as usize,
                        None => {
                            print!("{out}");
                            return;
                        }
                    };
                    out.push_str(&format!("{}\n", sh.maxk_from_front(k)));
                }
                _ => {
                    // Undefined per spec; stop to avoid panic.
                    print!("{out}");
                    return;
                }
            }
        }

        out.push_str("end\n");
    }

    print!("{out}");
}
