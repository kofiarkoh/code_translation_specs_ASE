use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Rect {
    height: i64,
    pos: i64,
}

fn large_histogram(heights: &[i64]) -> i64 {
    let w = heights.len();
    let mut max_area: i64 = 0;

    // Monotone increasing stack of (height, leftmost position where this height applies).
    let mut stack: Vec<Rect> = Vec::with_capacity(w + 1);

    // Iterate through all bars plus a sentinel 0 height to flush the stack.
    for i in 0..=w {
        let cur_h = if i == w { 0 } else { heights[i] };
        let mut start = i as i64;

        // Pop while the stack height is greater than current height.
        // This is the standard correct algorithm for largest rectangle in histogram.
        while let Some(top) = stack.last() {
            if top.height <= cur_h {
                break;
            }
            let pre = stack.pop().unwrap();
            let area = pre.height * ((i as i64) - pre.pos);
            if area > max_area {
                max_area = area;
            }
            start = pre.pos;
        }

        // Push current bar with the earliest start it can extend to.
        stack.push(Rect {
            height: cur_h,
            pos: start,
        });
    }

    max_area
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let w: usize = it.next().unwrap().parse().unwrap();
    let mut heights: Vec<i64> = Vec::with_capacity(w);
    for _ in 0..w {
        heights.push(it.next().unwrap().parse().unwrap());
    }

    let ans = if w == 0 { 0 } else { large_histogram(&heights) };
    print!("{ans}\n");
}
