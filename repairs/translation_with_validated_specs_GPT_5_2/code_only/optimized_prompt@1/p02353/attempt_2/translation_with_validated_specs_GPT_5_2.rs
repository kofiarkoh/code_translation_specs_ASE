use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

#[derive(Clone, Copy)]
struct Pair {
    lazy: i64,
    val: i64,
}

const COMMAND_UPDATE: i32 = 0;
const COMMAND_GET_SUM: i32 = 1;

const NUM2_MAX: usize = 1 << 17;
const QUERY_MAX: usize = 100000;

// C: #define INIT_VAL (2e+9)  -> double literal, exactly 2000000000 when converted to int64_t
const INIT_VAL: i64 = 2_000_000_000;

const ROOT_NODE: usize = 1;

fn is_overlap(r1: Range, r2: Range) -> bool {
    r1.end > r2.start && r2.end > r1.start
}

fn is_contain(this: Range, boxr: Range) -> bool {
    boxr.start <= this.start && this.end <= boxr.end
}

fn unlazy(seg: &mut [Pair], node: usize, nr: Range) {
    let lazy = seg[node].lazy;
    if lazy == INIT_VAL {
        return;
    }
    seg[node].val = lazy;
    seg[node].lazy = INIT_VAL;
    if nr.end - nr.start > 1 {
        seg[node * 2].lazy = lazy / 2;
        seg[node * 2 + 1].lazy = lazy / 2;
    }
}

fn update_range(seg: &mut [Pair], x: i32, r: Range, node: usize, nr: Range) {
    unlazy(seg, node, nr);
    if !is_overlap(r, nr) {
        return;
    }
    if is_contain(nr, r) {
        // Emulate C signed overflow as wrapping.
        let len = (nr.end - nr.start) as i64;
        seg[node].lazy = (x as i64).wrapping_mul(len);
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
    // Tokenize stdin similarly to scanf; if input is shorter than expected,
    // C would leave variables unchanged (UB-ish). Here we stop gracefully.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let Some(num_s) = it.next() else { return };
    let num: i32 = num_s.parse().unwrap_or(0);
    let Some(q_s) = it.next() else { return };
    let queries: i32 = q_s.parse().unwrap_or(0);

    let mut num2: i32 = 1;
    while num > num2 {
        num2 *= 2;
    }

    let mut ans: Vec<i64> = Vec::with_capacity(QUERY_MAX);
    let mut seg: Vec<Pair> = vec![Pair { lazy: 0, val: 0 }; NUM2_MAX * 2];

    let limit = (num2 as usize) * 2;
    for i in ROOT_NODE..limit {
        seg[i].lazy = INIT_VAL;
    }

    let whole = Range { start: 0, end: num2 };

    for _ in 0..queries {
        let Some(t_s) = it.next() else { break };
        let Some(st_s) = it.next() else { break };
        let Some(en_s) = it.next() else { break };

        let typev: i32 = t_s.parse().unwrap_or(0);
        let start: i32 = st_s.parse().unwrap_or(0);
        let end: i32 = en_s.parse().unwrap_or(0);

        // C does: struct range r = {start, end+1}; with int overflow UB.
        // Use wrapping_add to emulate typical 2's-complement behavior.
        let r = Range {
            start,
            end: end.wrapping_add(1),
        };

        match typev {
            COMMAND_UPDATE => {
                let Some(x_s) = it.next() else { break };
                let x: i32 = x_s.parse().unwrap_or(0);
                update_range(&mut seg, x, r, ROOT_NODE, whole);
            }
            COMMAND_GET_SUM => {
                let s = get_sum(&mut seg, r, ROOT_NODE, whole);
                ans.push(s);
            }
            _ => {}
        }
    }

    let mut out = String::new();
    for v in ans {
        out.push_str(&v.to_string());
        out.push('\n');
    }
    print!("{out}");
}
