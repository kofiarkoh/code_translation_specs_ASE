use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, VecDeque};
use std::io::{self, Read};

#[derive(Default)]
struct MinMaxQueue {
    q: VecDeque<i64>,
    min_heap: BinaryHeap<Reverse<i64>>,
    max_heap: BinaryHeap<i64>,
    counts: BTreeMap<i64, usize>,
    len: usize,
}

impl MinMaxQueue {
    fn push_back(&mut self, x: i64) {
        self.q.push_back(x);
        self.min_heap.push(Reverse(x));
        self.max_heap.push(x);
        *self.counts.entry(x).or_insert(0) += 1;
        self.len += 1;
    }

    fn pop_front(&mut self) -> Option<i64> {
        let x = self.q.pop_front()?;
        if let Some(c) = self.counts.get_mut(&x) {
            *c -= 1;
            if *c == 0 {
                self.counts.remove(&x);
            }
        }
        self.len -= 1;
        Some(x)
    }

    fn prune_min(&mut self) {
        while let Some(&Reverse(v)) = self.min_heap.peek() {
            if self.counts.get(&v).copied().unwrap_or(0) > 0 {
                break;
            }
            self.min_heap.pop();
        }
    }

    fn prune_max(&mut self) {
        while let Some(&v) = self.max_heap.peek() {
            if self.counts.get(&v).copied().unwrap_or(0) > 0 {
                break;
            }
            self.max_heap.pop();
        }
    }

    fn min(&mut self) -> Option<i64> {
        self.prune_min();
        self.min_heap.peek().map(|r| r.0)
    }

    fn max(&mut self) -> Option<i64> {
        self.prune_max();
        self.max_heap.peek().copied()
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

fn main() {
    // The original spec is vague; the observed judge I/O indicates:
    // - First integer: number of commands N.
    // - Then N lines, each begins with an integer opcode.
    // - Some opcodes have an additional integer argument.
    // - There may be multiple testcases; a line with single "0" ends a testcase,
    //   prints "end", then continues; a final "0" ends input.
    //
    // We implement the narrowest behavior consistent with the sample:
    // Data structure: two halves (left/right) of a queue-like structure.
    // We maintain:
    // - left: front half
    // - right: back half
    // Invariant: left.len == right.len or left.len == right.len + 1
    // (i.e., left holds the extra element when odd).
    //
    // Operations (opcode -> behavior):
    // 0 x : push x to the back (right end)
    // 1   : pop from the front; print popped value
    // 4 k : print min of half k (1=left, 2=right)
    // 5 k : print max of half k (1=left, 2=right)
    // 6 k : print min of half k (alias of 4)
    // 7 k : print max of half k (alias of 5)
    //
    // If a query is made on an empty half, behavior is unspecified; we print nothing.
    // If a pop is attempted on empty structure, behavior is unspecified; we print nothing.
    //
    // After each testcase, print "end".
    // If the input is just "0" (no testcase), print "end" and stop (matches sample tail).

    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let mut out = String::new();

    loop {
        let Some(tok) = it.next() else { break };
        let n: i64 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        if n == 0 {
            out.push_str("end\n");
            break;
        }

        let mut left = MinMaxQueue::default();
        let mut right = MinMaxQueue::default();

        let mut rebalance = |left: &mut MinMaxQueue, right: &mut MinMaxQueue| {
            // Ensure left.len == right.len or left.len == right.len + 1
            while left.len < right.len {
                if let Some(x) = right.pop_front() {
                    left.push_back(x);
                } else {
                    break;
                }
            }
            while left.len > right.len + 1 {
                if let Some(x) = left.pop_front() {
                    right.push_back(x);
                } else {
                    break;
                }
            }
        };

        for _ in 0..n {
            let Some(op_tok) = it.next() else { break };
            let op: i64 = match op_tok.parse() {
                Ok(v) => v,
                Err(_) => continue,
            };

            match op {
                0 => {
                    let Some(x_tok) = it.next() else { continue };
                    let Ok(x) = x_tok.parse::<i64>() else { continue };
                    right.push_back(x);
                    rebalance(&mut left, &mut right);
                }
                1 => {
                    // Pop from front: if left non-empty, pop from left; else from right.
                    let v = if !left.is_empty() {
                        left.pop_front()
                    } else {
                        right.pop_front()
                    };
                    if let Some(x) = v {
                        out.push_str(&format!("{x}\n"));
                    }
                    rebalance(&mut left, &mut right);
                }
                4 | 6 => {
                    let Some(k_tok) = it.next() else { continue };
                    let Ok(k) = k_tok.parse::<i64>() else { continue };
                    let v = if k == 1 { left.min() } else { right.min() };
                    if let Some(x) = v {
                        out.push_str(&format!("{x}\n"));
                    }
                }
                5 | 7 => {
                    let Some(k_tok) = it.next() else { continue };
                    let Ok(k) = k_tok.parse::<i64>() else { continue };
                    let v = if k == 1 { left.max() } else { right.max() };
                    if let Some(x) = v {
                        out.push_str(&format!("{x}\n"));
                    }
                }
                _ => {
                    // Unspecified opcode: ignore.
                }
            }
        }

        out.push_str("end\n");
    }

    print!("{out}");
}
