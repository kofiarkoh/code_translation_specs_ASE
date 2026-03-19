use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::io::{self, Read};

#[derive(Default)]
struct MultiSet {
    // value -> count
    m: BTreeMap<i64, usize>,
    len: usize,
}

impl MultiSet {
    fn new() -> Self {
        Self {
            m: BTreeMap::new(),
            len: 0,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn insert(&mut self, x: i64) {
        *self.m.entry(x).or_insert(0) += 1;
        self.len += 1;
    }

    fn remove_one(&mut self, x: i64) {
        // Preconditions guarantee x exists.
        if let Some(c) = self.m.get_mut(&x) {
            *c -= 1;
            self.len -= 1;
            if *c == 0 {
                self.m.remove(&x);
            }
        }
    }

    fn min(&self) -> i64 {
        // Preconditions guarantee non-empty.
        *self.m.keys().next().unwrap()
    }

    fn max(&self) -> i64 {
        // Preconditions guarantee non-empty.
        *self.m.keys().next_back().unwrap()
    }

    fn kth_smallest(&self, mut k: usize) -> i64 {
        // k is 1-based; Preconditions guarantee 1 <= k <= len.
        for (&v, &c) in self.m.iter() {
            if k <= c {
                return v;
            }
            k -= c;
        }
        // Undefined behavior per spec if preconditions violated; keep a hard panic.
        panic!("kth_smallest out of range");
    }

    fn kth_largest(&self, mut k: usize) -> i64 {
        // k is 1-based; Preconditions guarantee 1 <= k <= len.
        for (&v, &c) in self.m.iter().rev() {
            if k <= c {
                return v;
            }
            k -= c;
        }
        panic!("kth_largest out of range");
    }
}

struct State {
    lower: MultiSet, // lower half: size == upper or upper+1
    upper: MultiSet, // upper half
}

impl State {
    fn new() -> Self {
        Self {
            lower: MultiSet::new(),
            upper: MultiSet::new(),
        }
    }

    fn rebalance(&mut self) {
        // Enforce size invariant: lower.len == upper.len or lower.len == upper.len + 1
        while self.lower.len() > self.upper.len() + 1 {
            let x = self.lower.max();
            self.lower.remove_one(x);
            self.upper.insert(x);
        }
        while self.lower.len() < self.upper.len() {
            let x = self.upper.min();
            self.upper.remove_one(x);
            self.lower.insert(x);
        }

        // Enforce ordering invariant: max(lower) <= min(upper) when both non-empty.
        if self.lower.len() > 0 && self.upper.len() > 0 {
            let a = self.lower.max();
            let b = self.upper.min();
            if a > b {
                // Swap one occurrence to restore ordering.
                self.lower.remove_one(a);
                self.upper.remove_one(b);
                self.lower.insert(b);
                self.upper.insert(a);
                // Sizes unchanged; ordering should now hold, but keep it robust.
                // (No extra rebalancing needed for sizes.)
            }
        }
    }

    fn insert(&mut self, x: i64) {
        // Insert then re-establish partition.
        if self.lower.len() == 0 {
            self.lower.insert(x);
        } else {
            let lm = self.lower.max();
            if x <= lm {
                self.lower.insert(x);
            } else {
                self.upper.insert(x);
            }
        }
        self.rebalance();
    }

    fn remove_median(&mut self) -> i64 {
        // Median removal: remove and output max(lower) (lower median).
        let x = self.lower.max();
        self.lower.remove_one(x);
        self.rebalance();
        x
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let q_opt = it.next();
        if q_opt.is_none() {
            break;
        }
        let q: usize = q_opt.unwrap().parse().unwrap();
        if q == 0 {
            break;
        }

        let mut st = State::new();

        for _ in 0..q {
            let op: u64 = it.next().unwrap().parse().unwrap();
            match op {
                0 => {
                    let x: i64 = it.next().unwrap().parse().unwrap();
                    st.insert(x);
                }
                1 => {
                    let v = st.remove_median();
                    out.push_str(&format!("{v}\n"));
                }
                2 => {
                    // smallest in lower half
                    let v = st.lower.kth_smallest(1);
                    out.push_str(&format!("{v}\n"));
                }
                3 => {
                    // smallest in upper half
                    let v = st.upper.kth_smallest(1);
                    out.push_str(&format!("{v}\n"));
                }
                4 => {
                    let k: usize = it.next().unwrap().parse().unwrap();
                    let v = st.lower.kth_smallest(k);
                    out.push_str(&format!("{v}\n"));
                }
                5 => {
                    let k: usize = it.next().unwrap().parse().unwrap();
                    let v = st.upper.kth_smallest(k);
                    out.push_str(&format!("{v}\n"));
                }
                6 => {
                    // largest in lower half
                    let v = st.lower.kth_largest(1);
                    out.push_str(&format!("{v}\n"));
                }
                7 => {
                    // largest in upper half
                    let v = st.upper.kth_largest(1);
                    out.push_str(&format!("{v}\n"));
                }
                8 => {
                    let k: usize = it.next().unwrap().parse().unwrap();
                    let v = st.lower.kth_largest(k);
                    out.push_str(&format!("{v}\n"));
                }
                _ => {
                    // 9 k and any other code: k-th largest in upper half
                    let k: usize = it.next().unwrap().parse().unwrap();
                    let v = st.upper.kth_largest(k);
                    out.push_str(&format!("{v}\n"));
                }
            }
        }

        out.push_str("end\n");
    }

    print!("{out}");
}
