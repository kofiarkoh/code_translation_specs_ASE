use std::io::{self, Read};

const INIT_VAL: i64 = 2_000_000_000; // matches C's (int64_t)(2e+9)
const ROOT_NODE: usize = 1;

#[derive(Clone, Copy)]
struct Range {
    // Half-open interval [start, end)
    start: i32,
    end: i32,
}

#[derive(Clone, Copy)]
struct Pair {
    lazy: i64, // pending segment-sum assignment marker; INIT_VAL means "none"
    val: i64,  // segment sum
}

fn is_overlap(r1: Range, r2: Range) -> bool {
    // C uses strict comparisons for half-open intervals
    r1.end > r2.start && r2.end > r1.start
}

fn is_contain(this: Range, box_: Range) -> bool {
    // "this" is contained in "box_"
    box_.start <= this.start && this.end <= box_.end
}

fn unlazy(seg: &mut [Pair], node: usize, nr: Range) {
    let lazy = seg[node].lazy;
    if lazy == INIT_VAL {
        return;
    }
    seg[node].val = lazy; // apply pending segment sum
    seg[node].lazy = INIT_VAL;

    // If not a leaf (length > 1), propagate to children.
    if (nr.end - nr.start) > 1 {
        // In this implementation, lazy stores the *segment sum* for this node.
        // Children each get half the sum (tree is built on power-of-two length).
        let child_lazy = lazy / 2;
        seg[node * 2].lazy = child_lazy;
        seg[node * 2 + 1].lazy = child_lazy;
    }
}

fn update_range(seg: &mut [Pair], x: i32, r: Range, node: usize, nr: Range) {
    unlazy(seg, node, nr);
    if !is_overlap(r, nr) {
        return;
    }
    if is_contain(nr, r) {
        // Store segment sum for this node as lazy marker, then apply immediately.
        let len = (nr.end - nr.start) as i64;
        seg[node].lazy = (x as i64) * len;
        unlazy(seg, node, nr);
        return;
    }

    let mid = (nr.start + nr.end) / 2;
    let left = Range {
        start: nr.start,
        end: mid,
    };
    let right = Range { start: mid, end: nr.end };

    update_range(seg, x, r, node * 2, left);
    update_range(seg, x, r, node * 2 + 1, right);
    seg[node].val = seg[node * 2].val + seg[node * 2 + 1].val;
}

fn get_sum(seg: &mut [Pair], r: Range, node: usize, nr: Range) -> i64 {
    unlazy(seg, node, nr);
    if !is_overlap(r, nr) {
        return 0;
    }
    if is_contain(nr, r) {
        return seg[node].val;
    }

    let mid = (nr.start + nr.end) / 2;
    let left = Range {
        start: nr.start,
        end: mid,
    };
    let right = Range { start: mid, end: nr.end };

    get_sum(seg, r, node * 2, left) + get_sum(seg, r, node * 2 + 1, right)
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let num: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let queries: i32 = it.next().unwrap().parse().unwrap();

    // Build segment tree over num2 = next power of two >= num.
    let mut num2: i32 = 1;
    while num > num2 {
        num2 *= 2;
    }

    // seg size: 2*num2 (1-indexed nodes used), like C's static array.
    let size = (num2 as usize) * 2;
    let mut seg = vec![
        Pair {
            lazy: INIT_VAL,
            val: 0
        };
        size
    ];

    // C initializes lazy for i in [ROOT_NODE, 2*num2)
    for i in ROOT_NODE..size {
        seg[i].lazy = INIT_VAL;
    }

    let whole = Range { start: 0, end: num2 };

    let mut outputs: Vec<i64> = Vec::new();
    for _ in 0..queries {
        let type_: i32 = it.next().unwrap().parse().unwrap();
        let start: i32 = it.next().unwrap().parse().unwrap();
        let end: i32 = it.next().unwrap().parse().unwrap();

        // Spec uses inclusive [start,end]; internal uses half-open [start,end+1)
        let r = Range {
            start,
            end: end + 1,
        };

        match type_ {
            0 => {
                let x: i32 = it.next().unwrap().parse().unwrap();
                update_range(&mut seg, x, r, ROOT_NODE, whole);
            }
            1 => {
                let s = get_sum(&mut seg, r, ROOT_NODE, whole);
                outputs.push(s);
            }
            _ => {
                // Unrecognized operation: ignored (no output, no state change).
            }
        }
    }

    let mut out = String::new();
    for v in outputs {
        out.push_str(&format!("{v}\n"));
    }
    print!("{out}");
}
