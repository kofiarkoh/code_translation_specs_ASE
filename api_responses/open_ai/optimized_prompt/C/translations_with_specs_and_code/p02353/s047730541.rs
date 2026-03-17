use std::io::{self, BufRead};

const INIT_VAL: i64 = 2_000_000_000;
const NUM2_MAX: usize = 1 << 17;
const ROOT_NODE: usize = 1;

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

#[derive(Default)]
struct Pair {
    lazy: i64,
    val: i64,
}

struct SegmentTree {
    seg: Vec<Pair>,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let mut seg = vec![Pair::default(); size * 2];
        for pair in seg.iter_mut() {
            pair.lazy = INIT_VAL;
        }
        SegmentTree { seg }
    }

    fn unlazy(&mut self, node: usize, nr: Range) {
        let lazy = self.seg[node].lazy;
        if lazy == INIT_VAL {
            return;
        }
        self.seg[node].val = lazy; // update
        self.seg[node].lazy = INIT_VAL;
        if nr.end - nr.start > 1 {
            self.seg[node * 2].lazy = lazy / 2;
            self.seg[node * 2 + 1].lazy = lazy / 2;
        }
    }

    fn update_range(&mut self, x: i64, r: Range, node: usize, nr: Range) {
        self.unlazy(node, nr);
        if !self.is_overlap(r, nr) {
            return;
        }
        if self.is_contain(nr, r) {
            self.seg[node].lazy = x * (nr.end - nr.start) as i64;
            self.unlazy(node, nr);
            return;
        }
        let mid = (nr.start + nr.end) / 2;
        let left = Range { start: nr.start, end: mid };
        let right = Range { start: mid, end: nr.end };
        self.update_range(x, r, node * 2, left);
        self.update_range(x, r, node * 2 + 1, right);
        self.seg[node].val = self.seg[node * 2].val + self.seg[node * 2 + 1].val;
    }

    fn get_sum(&mut self, r: Range, node: usize, nr: Range) -> i64 {
        self.unlazy(node, nr);
        if !self.is_overlap(r, nr) {
            return 0;
        }
        if self.is_contain(nr, r) {
            return self.seg[node].val;
        }
        let mid = (nr.start + nr.end) / 2;
        let left = Range { start: nr.start, end: mid };
        let right = Range { start: mid, end: nr.end };
        self.get_sum(r, node * 2, left) + self.get_sum(r, node * 2 + 1, right)
    }

    fn is_overlap(&self, r1: Range, r2: Range) -> bool {
        r1.end > r2.start && r2.end > r1.start
    }

    fn is_contain(&self, this: Range, box_: Range) -> bool {
        box_.start <= this.start && this.end <= box_.end
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().map(|line| line.unwrap());

    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let num: usize = parts.next().unwrap().parse().unwrap();
    let queries: usize = parts.next().unwrap().parse().unwrap();

    let mut seg_tree = SegmentTree::new(NUM2_MAX);
    let whole = Range { start: 0, end: num.next_power_of_two() };

    let mut results = Vec::new();

    for _ in 0..queries {
        let query_line = lines.next().unwrap();
        let mut query_parts = query_line.split_whitespace();
        let query_type: i64 = query_parts.next().unwrap().parse().unwrap();
        let start: usize = query_parts.next().unwrap().parse().unwrap();
        let end: usize = query_parts.next().unwrap().parse().unwrap();
        let r = Range { start, end: end + 1 };

        match query_type {
            0 => { // COMMAND_UPDATE
                let x: i64 = lines.next().unwrap().parse().unwrap();
                seg_tree.update_range(x, r, ROOT_NODE, whole);
            }
            1 => { // COMMAND_GET_SUM
                let sum = seg_tree.get_sum(r, ROOT_NODE, whole);
                results.push(sum);
            }
            _ => {}
        }
    }

    for result in results {
        println!("{}", result);
    }
}
