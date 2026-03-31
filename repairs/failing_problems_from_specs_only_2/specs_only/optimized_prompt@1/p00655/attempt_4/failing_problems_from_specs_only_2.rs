use std::collections::VecDeque;
use std::io::{self, Read};

#[derive(Default)]
struct FH {
    stack: Vec<i32>,    // LIFO
    min_hist: Vec<i32>, // strictly decreasing record of new minima
    max_hist: Vec<i32>, // strictly increasing record of new maxima
}

impl FH {
    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if self.min_hist.last().map_or(true, |&m| x < m) {
            self.min_hist.push(x);
        }
        if self.max_hist.last().map_or(true, |&m| x > m) {
            self.max_hist.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        let x = self.stack.pop().unwrap();
        if self.min_hist.last().copied() == Some(x) {
            self.min_hist.pop();
        }
        if self.max_hist.last().copied() == Some(x) {
            self.max_hist.pop();
        }
        x
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn min_k_from_end(&self, k: usize) -> i32 {
        self.min_hist[self.min_hist.len() - k]
    }

    fn max_k_from_end(&self, k: usize) -> i32 {
        self.max_hist[self.max_hist.len() - k]
    }
}

#[derive(Default)]
struct SH {
    q: VecDeque<i32>,        // FIFO
    min_mono: VecDeque<i32>, // non-decreasing
    max_mono: VecDeque<i32>, // non-increasing
}

impl SH {
    fn push_back(&mut self, x: i32) {
        self.q.push_back(x);

        while let Some(&b) = self.min_mono.back() {
            if b > x {
                self.min_mono.pop_back();
            } else {
                break;
            }
        }
        self.min_mono.push_back(x);

        while let Some(&b) = self.max_mono.back() {
            if b < x {
                self.max_mono.pop_back();
            } else {
                break;
            }
        }
        self.max_mono.push_back(x);
    }

    fn pop_front(&mut self) -> i32 {
        let x = self.q.pop_front().unwrap();
        if self.min_mono.front().copied() == Some(x) {
            self.min_mono.pop_front();
        }
        if self.max_mono.front().copied() == Some(x) {
            self.max_mono.pop_front();
        }
        x
    }

    fn len(&self) -> usize {
        self.q.len()
    }

    fn min_k_from_front(&self, k: usize) -> i32 {
        self.min_mono[k - 1]
    }

    fn max_k_from_front(&self, k: usize) -> i32 {
        self.max_mono[k - 1]
    }
}

fn rebalance_if_needed(fh: &mut FH, sh: &mut SH) {
    if fh.len() < sh.len() {
        let x = sh.pop_front();
        fh.push(x);
    }
}

struct Scanner {
    data: Vec<u8>,
    idx: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        Self {
            data: s.into_bytes(),
            idx: 0,
        }
    }

    fn next_i32(&mut self) -> Option<i32> {
        while self.idx < self.data.len() && self.data[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        if self.idx >= self.data.len() {
            return None;
        }

        let mut sign = 1i32;
        if self.data[self.idx] == b'-' {
            sign = -1;
            self.idx += 1;
        }

        let mut val: i32 = 0;
        let mut any = false;
        while self.idx < self.data.len() && !self.data[self.idx].is_ascii_whitespace() {
            let c = self.data[self.idx];
            // If malformed token occurs, treat as end-of-input to avoid panics.
            if !(b'0'..=b'9').contains(&c) {
                return None;
            }
            any = true;
            val = val * 10 + (c - b'0') as i32;
            self.idx += 1;
        }
        if !any {
            None
        } else {
            Some(val * sign)
        }
    }

    fn next_usize(&mut self) -> Option<usize> {
        self.next_i32().map(|v| v as usize)
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    loop {
        let Some(q) = sc.next_usize() else { break };
        if q == 0 {
            break;
        }

        let mut fh = FH::default();
        let mut sh = SH::default();

        for _ in 0..q {
            let Some(op) = sc.next_i32() else {
                print!("{out}");
                return;
            };

            match op {
                0 => {
                    let Some(x) = sc.next_i32() else {
                        print!("{out}");
                        return;
                    };
                    sh.push_back(x);
                    rebalance_if_needed(&mut fh, &mut sh);
                }
                1 => {
                    let v = fh.pop();
                    out.push_str(&v.to_string());
                    out.push('\n');
                    rebalance_if_needed(&mut fh, &mut sh);
                }
                2 => {
                    let v = fh.min_k_from_end(1);
                    out.push_str(&v.to_string());
                    out.push('\n');
                }
                3 => {
                    let v = sh.min_k_from_front(1);
                    out.push_str(&v.to_string());
                    out.push('\n');
                }
                4 => {
                    let Some(k) = sc.next_usize() else {
                        print!("{out}");
                        return;
                    };
                    let v = fh.min_k_from_end(k);
                    out.push_str(&v.to_string());
                    out.push('\n');
                }
                5 => {
                    let Some(k) = sc.next_usize() else {
                        print!("{out}");
                        return;
                    };
                    let v = sh.min_k_from_front(k);
                    out.push_str(&v.to_string());
                    out.push('\n');
                }
                6 => {
                    let v = fh.max_k_from_end(1);
                    out.push_str(&v.to_string());
                    out.push('\n');
                }
                7 => {
                    let v = sh.max_k_from_front(1);
                    out.push_str(&v.to_string());
                    out.push('\n');
                }
                8 => {
                    let Some(k) = sc.next_usize() else {
                        print!("{out}");
                        return;
                    };
                    let v = fh.max_k_from_end(k);
                    out.push_str(&v.to_string());
                    out.push('\n');
                }
                9 => {
                    let Some(k) = sc.next_usize() else {
                        print!("{out}");
                        return;
                    };
                    let v = sh.max_k_from_front(k);
                    out.push_str(&v.to_string());
                    out.push('\n');
                }
                _ => {
                    // Undefined behavior per spec; ignore.
                }
            }
        }

        out.push_str("end\n");
    }

    print!("{out}");
}
