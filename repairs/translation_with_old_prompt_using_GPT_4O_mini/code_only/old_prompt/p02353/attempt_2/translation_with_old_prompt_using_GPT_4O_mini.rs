use std::io::{self, BufRead};
use std::str::FromStr;

const MAX_NUM: usize = 100_000;
const NUM2_MAX: usize = 1 << 17;
const QUERY_MAX: usize = 100_000;
const INIT_VAL: i64 = 2_000_000_000;

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

#[derive(Default, Clone)]
struct Pair {
    lazy: i64,
    val: i64,
}

fn is_overlap(r1: &Range, r2: &Range) -> bool {
    r1.end > r2.start && r2.end > r1.start
}

fn is_contain(this: &Range, box_range: &Range) -> bool {
    box_range.start <= this.start && this.end <= box_range.end
}

fn unlazy(seg: &mut [Pair], node: usize, nr: &Range) {
    let lazy = seg[node].lazy;
    if lazy == INIT_VAL {
        return;
    }
    seg[node].val = lazy; // update
    seg[node].lazy = INIT_VAL;
    if nr.end - nr.start > 1 {
        seg[node * 2].lazy = lazy / 2;
        seg[node * 2 + 1].lazy = lazy / 2;
    }
}

fn update_range(seg: &mut [Pair], x: i64, r: &Range, node: usize, nr: &Range) {
    unlazy(seg, node, nr);
    if !is_overlap(r, nr) {
        return;
    }
    if is_contain(nr, r) {
        seg[node].lazy = x * (nr.end - nr.start) as i64;
        unlazy(seg, node, nr);
        return;
    }
    let left = Range {
        start: nr.start,
        end: (nr.start + nr.end) / 2,
    };
    let right = Range {
        start: (nr.start + nr.end) / 2,
        end: nr.end,
    };
    update_range(seg, x, r, node * 2, &left);
    update_range(seg, x, r, node * 2 + 1, &right);
    seg[node].val = seg[node * 2].val + seg[node * 2 + 1].val;
}

fn get_sum(seg: &mut [Pair], r: &Range, node: usize, nr: &Range) -> i64 {
    unlazy(seg, node, nr);
    if !is_overlap(r, nr) {
        return 0;
    }
    if is_contain(nr, r) {
        return seg[node].val;
    }
    let left = Range {
        start: nr.start,
        end: (nr.start + nr.end) / 2,
    };
    let right = Range {
        start: (nr.start + nr.end) / 2,
        end: nr.end,
    };
    let mut ans = 0;
    ans += get_sum(seg, r, node * 2, &left);
    ans += get_sum(seg, r, node * 2 + 1, &right);
    ans
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().map(|line| line.unwrap());

    let first_line: Vec<i32> = lines.next().unwrap().split_whitespace()
        .map(|s| i32::from_str(s).unwrap()).collect();
    let (mut num, mut queries) = (first_line[0] as usize, first_line[1] as usize);

    let mut num2 = 1;
    let mut ans = vec![0; QUERY_MAX];
    let mut aidx = 0;
    while num > num2 {
        num2 *= 2;
    }
    let mut seg = vec![Pair::default(); NUM2_MAX * 2];
    for i in 1..num2 * 2 {
        seg[i].lazy = INIT_VAL;
    }

    let whole = Range { start: 0, end: num2 };
    for _ in 0..queries {
        let query_line: Vec<i32> = lines.next().unwrap().split_whitespace()
            .map(|s| i32::from_str(s).unwrap()).collect();
        let (type_cmd, start, end) = (query_line[0], query_line[1], query_line[2]);
        let r = Range { start: start as usize, end: (end + 1) as usize };
        match type_cmd {
            0 => {
                let x = lines.next().unwrap().parse::<i64>().unwrap();
                update_range(&mut seg, x, &r, 1, &whole);
            }
            1 => {
                ans[aidx] = get_sum(&mut seg, &r, 1, &whole);
                aidx += 1;
            }
            _ => {}
        }
    }

    for i in 0..aidx {
        println!("{}", ans[i]);
    }
}
