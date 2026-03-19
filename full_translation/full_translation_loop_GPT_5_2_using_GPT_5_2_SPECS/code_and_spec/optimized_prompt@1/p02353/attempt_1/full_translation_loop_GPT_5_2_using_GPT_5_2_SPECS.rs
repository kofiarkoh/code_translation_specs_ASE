use std::io::{self, Read};

const COMMAND_UPDATE: i32 = 0;
const COMMAND_GET_SUM: i32 = 1;

// C uses INIT_VAL as a sentinel for "no lazy".
// It is defined as (2e+9) which is 2000000000.0, then used in int64_t context.
// We'll use the exact integer value 2_000_000_000.
const INIT_LAZY: i64 = 2_000_000_000;

#[derive(Clone, Copy)]
struct Node {
    lazy: i64, // when set: stores "assigned_value * segment_length" for this node
    val: i64,  // sum over the segment
}

#[derive(Clone, Copy)]
struct Range {
    start: i32,
    end: i32, // half-open [start, end)
}

fn is_overlap(r1: Range, r2: Range) -> bool {
    // Matches C: r1.end > r2.start && r2.end > r1.start
    r1.end > r2.start && r2.end > r1.start
}

fn is_contain(this: Range, box_: Range) -> bool {
    // Matches C: box.start <= this.start && this.end <= box.end
    box_.start <= this.start && this.end <= box_.end
}

fn unlazy(seg: &mut [Node], node: usize, nr: Range) {
    let lazy = seg[node].lazy;
    if lazy == INIT_LAZY {
        return;
    }
    // Apply lazy to this node
    seg[node].val = lazy;
    seg[node].lazy = INIT_LAZY;

    // Propagate to children if not a leaf (segment length > 1)
    if (nr.end - nr.start) > 1 {
        let half = lazy / 2; // C divides by 2; relies on evenness (len is power of two)
        seg[node * 2].lazy = half;
        seg[node * 2 + 1].lazy = half;
    }
}

fn update_range(seg: &mut [Node], x: i64, r: Range, node: usize, nr: Range) {
    unlazy(seg, node, nr);
    if !is_overlap(r, nr) {
        return;
    }
    if is_contain(nr, r) {
        let len = (nr.end - nr.start) as i64;
        seg[node].lazy = x * len;
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

fn get_sum(seg: &mut [Node], r: Range, node: usize, nr: Range) -> i64 {
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
    // Read all input (no explicit error handling per spec)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let num: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let queries: i32 = it.next().unwrap().parse().unwrap();

    // Next power of two >= num
    let mut num2: i32 = 1;
    while num > num2 {
        num2 *= 2;
    }

    // Segment tree array: indices [1 .. 2*num2)
    let size = (num2 as usize) * 2;
    let mut seg = vec![
        Node {
            lazy: INIT_LAZY,
            val: 0
        };
        size
    ];
    // In C, seg[0] is unused; we keep it initialized too.

    let whole = Range { start: 0, end: num2 };
    let mut answers: Vec<i64> = Vec::new();
    answers.reserve(100_000);

    for _ in 0..queries {
        let type_: i32 = it.next().unwrap().parse().unwrap();
        let start: i32 = it.next().unwrap().parse().unwrap();
        let end: i32 = it.next().unwrap().parse().unwrap();

        // C converts inclusive [start,end] to half-open [start,end+1)
        let r = Range {
            start,
            end: end + 1,
        };

        match type_ {
            COMMAND_UPDATE => {
                let x: i64 = it.next().unwrap().parse::<i32>().unwrap() as i64;
                update_range(&mut seg, x, r, 1, whole);
            }
            COMMAND_GET_SUM => {
                let s = get_sum(&mut seg, r, 1, whole);
                answers.push(s);
            }
            _ => {
                // Per spec/C: unsupported type => no output, no modification
            }
        }
    }

    let mut out = String::new();
    for v in answers {
        out.push_str(&format!("{v}\n"));
    }
    print!("{out}");
}
