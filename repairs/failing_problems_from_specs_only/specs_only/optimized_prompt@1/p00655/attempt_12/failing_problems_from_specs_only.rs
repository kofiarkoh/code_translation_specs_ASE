use std::collections::VecDeque;
use std::io::{self, Read};

#[derive(Default)]
struct FH {
    stack: Vec<i32>,
    min_hist: Vec<i32>,
    max_hist: Vec<i32>,
}

impl FH {
    fn clear(&mut self) {
        self.stack.clear();
        self.min_hist.clear();
        self.max_hist.clear();
    }

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
        if self.min_hist.last().map_or(false, |&m| m == x) {
            self.min_hist.pop();
        }
        if self.max_hist.last().map_or(false, |&m| m == x) {
            self.max_hist.pop();
        }
        x
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn query_min_k(&self, k: usize) -> i32 {
        self.min_hist[self.min_hist.len() - k]
    }

    fn query_max_k(&self, k: usize) -> i32 {
        self.max_hist[self.max_hist.len() - k]
    }
}

#[derive(Default)]
struct SH {
    q: VecDeque<i32>,
    min_mono: VecDeque<i32>,
    max_mono: VecDeque<i32>,
}

impl SH {
    fn clear(&mut self) {
        self.q.clear();
        self.min_mono.clear();
        self.max_mono.clear();
    }

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
        if self.min_mono.front().map_or(false, |&v| v == x) {
            self.min_mono.pop_front();
        }
        if self.max_mono.front().map_or(false, |&v| v == x) {
            self.max_mono.pop_front();
        }
        x
    }

    fn len(&self) -> usize {
        self.q.len()
    }

    fn query_min_k(&self, k: usize) -> i32 {
        self.min_mono[k - 1]
    }

    fn query_max_k(&self, k: usize) -> i32 {
        self.max_mono[k - 1]
    }
}

fn rebalance_if_needed(fh: &mut FH, sh: &mut SH) {
    if fh.len() < sh.len() {
        let x = sh.pop_front();
        fh.push(x);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();
    let mut fh = FH::default();
    let mut sh = SH::default();

    loop {
        let Some(qs) = it.next() else { break };
        let q: usize = qs.parse().unwrap();

        // Sample expects that the final "0" is echoed and followed by "end".
        if q == 0 {
            out.push_str("0\nend\n");
            break;
        }

        fh.clear();
        sh.clear();

        for _ in 0..q {
            let Some(op_s) = it.next() else {
                // Avoid panicking on unexpected EOF.
                print!("{out}");
                return;
            };
            let op: i32 = op_s.parse().unwrap();

            match op {
                0 => {
                    let Some(xs) = it.next() else {
                        print!("{out}");
                        return;
                    };
                    let x: i32 = xs.parse().unwrap();
                    sh.push_back(x);
                    rebalance_if_needed(&mut fh, &mut sh);
                }
                1 => {
                    if fh.len() == 0 {
                        print!("{out}");
                        return;
                    }
                    let v = fh.pop();
                    rebalance_if_needed(&mut fh, &mut sh);
                    out.push_str(&format!("{v}\n"));
                }
                2 => {
                    if fh.min_hist.is_empty() {
                        print!("{out}");
                        return;
                    }
                    out.push_str(&format!("{}\n", fh.query_min_k(1)));
                }
                3 => {
                    if sh.min_mono.is_empty() {
                        print!("{out}");
                        return;
                    }
                    out.push_str(&format!("{}\n", sh.query_min_k(1)));
                }
                4 => {
                    let Some(ks) = it.next() else {
                        print!("{out}");
                        return;
                    };
                    let k: usize = ks.parse().unwrap();
                    if k == 0 || k > fh.min_hist.len() {
                        print!("{out}");
                        return;
                    }
                    out.push_str(&format!("{}\n", fh.query_min_k(k)));
                }
                5 => {
                    let Some(ks) = it.next() else {
                        print!("{out}");
                        return;
                    };
                    let k: usize = ks.parse().unwrap();
                    if k == 0 || k > sh.min_mono.len() {
                        print!("{out}");
                        return;
                    }
                    out.push_str(&format!("{}\n", sh.query_min_k(k)));
                }
                6 => {
                    if fh.max_hist.is_empty() {
                        print!("{out}");
                        return;
                    }
                    out.push_str(&format!("{}\n", fh.query_max_k(1)));
                }
                7 => {
                    if sh.max_mono.is_empty() {
                        print!("{out}");
                        return;
                    }
                    out.push_str(&format!("{}\n", sh.query_max_k(1)));
                }
                8 => {
                    let Some(ks) = it.next() else {
                        print!("{out}");
                        return;
                    };
                    let k: usize = ks.parse().unwrap();
                    if k == 0 || k > fh.max_hist.len() {
                        print!("{out}");
                        return;
                    }
                    out.push_str(&format!("{}\n", fh.query_max_k(k)));
                }
                9 => {
                    let Some(ks) = it.next() else {
                        print!("{out}");
                        return;
                    };
                    let k: usize = ks.parse().unwrap();
                    if k == 0 || k > sh.max_mono.len() {
                        print!("{out}");
                        return;
                    }
                    out.push_str(&format!("{}\n", sh.query_max_k(k)));
                }
                _ => unreachable!(),
            }
        }

        out.push_str("end\n");
    }

    print!("{out}");
}
