use std::io::{self, Read};

const COMMAND_UPDATE: i32 = 0;
const COMMAND_GET_SUM: i32 = 1;

#[derive(Copy, Clone)]
struct Range {
    start: i32,
    end: i32,
}

#[derive(Copy, Clone)]
struct Pair {
    lazy: i64,
    val: i64,
}

const NUM2_MAX: usize = 1 << 17;
const QUERY_MAX: usize = 100000;

// C: #define INIT_VAL (2e+9)  -> double literal, exactly 2_000_000_000, used as sentinel in int64_t
const INIT_VAL: i64 = 2_000_000_000;

fn is_overlap(r1: Range, r2: Range) -> bool {
    r1.end > r2.start && r2.end > r1.start
}

fn is_contain(this: Range, box_: Range) -> bool {
    box_.start <= this.start && this.end <= box_.end
}

fn unlazy(seg: &mut [Pair], node: usize, nr: Range) {
    let lazy = seg[node].lazy;
    if lazy == INIT_VAL {
        return;
    }
    seg[node].val = lazy;
    seg[node].lazy = INIT_VAL;
    if nr.end - nr.start > 1 {
        let half = lazy / 2;
        seg[node * 2].lazy = half;
        seg[node * 2 + 1].lazy = half;
    }
}

fn update_range(seg: &mut [Pair], x: i32, r: Range, node: usize, nr: Range) {
    unlazy(seg, node, nr);
    if !is_overlap(r, nr) {
        return;
    }
    if is_contain(nr, r) {
        // C does int multiplication then assigns to int64_t; emulate 32-bit wrap.
        let len_i32 = nr.end.wrapping_sub(nr.start);
        let prod_i32 = x.wrapping_mul(len_i32);
        seg[node].lazy = prod_i32 as i64;
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
    seg[node].val = seg[node * 2].val.wrapping_add(seg[node * 2 + 1].val);
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
    let a = get_sum(seg, r, node * 2, left);
    let b = get_sum(seg, r, node * 2 + 1, right);
    a.wrapping_add(b)
}

fn main() {
    // Read all input; unlike unwrap()-based parsing, tolerate trailing whitespace / missing tokens
    // similarly to how scanf would simply stop reading.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let num: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let queries: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let mut num2: i32 = 1;
    while num > num2 {
        num2 *= 2;
    }

    let mut ans: Vec<i64> = Vec::with_capacity(QUERY_MAX);
    let mut seg = vec![
        Pair {
            lazy: 0,
            val: 0,
        };
        NUM2_MAX * 2
    ];

    let root_node: usize = 1;
    let limit = (num2 as usize) * 2;
    for i in root_node..limit {
        seg[i].lazy = INIT_VAL;
    }

    let whole = Range { start: 0, end: num2 };

    for _ in 0..queries {
        let type_: i32 = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break,
        };
        let start: i32 = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break,
        };
        let end: i32 = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break,
        };

        let r = Range {
            start,
            end: end + 1,
        };

        match type_ {
            COMMAND_UPDATE => {
                let x: i32 = match it.next() {
                    Some(v) => v.parse().unwrap(),
                    None => break,
                };
                update_range(&mut seg, x, r, root_node, whole);
            }
            COMMAND_GET_SUM => {
                let s = get_sum(&mut seg, r, root_node, whole);
                ans.push(s);
            }
            _ => {
                // C default: do nothing
            }
        }
    }

    let mut out = String::new();
    for v in ans {
        out.push_str(&v.to_string());
        out.push('\n');
    }
    print!("{out}");
}
