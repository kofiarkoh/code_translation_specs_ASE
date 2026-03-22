use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior on missing/unparseable input is unspecified in the spec.
    // We keep parsing simple and will panic on malformed input, which is acceptable under
    // the "undefined behavior" allowance for invalid input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let w: i64 = it.next().unwrap().parse().unwrap();

    // Spec bounds: 0 <= w <= 100000. If violated, behavior is undefined; we do not validate.
    let w_usize = w as usize;

    // Heights plus a trailing sentinel 0 at index w.
    let mut h: Vec<i64> = Vec::with_capacity(w_usize + 1);
    for _ in 0..w_usize {
        let v: i64 = it.next().unwrap().parse().unwrap();
        h.push(v);
    }
    h.push(0); // sentinel to flush the stack

    // Monotonic increasing stack of (height, earliest_start_index).
    let mut stack: Vec<(i64, usize)> = Vec::with_capacity(w_usize + 1);

    let mut best: i64 = 0;

    for i in 0..=w_usize {
        let cur_h = h[i];
        let mut start = i;

        // Finalize rectangles that cannot extend to i due to lower current height.
        while let Some(&(top_h, top_start)) = stack.last() {
            if top_h <= cur_h {
                break;
            }
            stack.pop();
            start = top_start;

            // Area computed in signed long long; overflow is undefined in the spec.
            // We use wrapping arithmetic to avoid Rust panics in debug builds and to
            // keep behavior low-level; for valid inputs (no overflow), this matches.
            let width = (i - top_start) as i64;
            let area = top_h.wrapping_mul(width);
            if area > best {
                best = area;
            }
        }

        // Maintain strictly increasing heights; merge equal heights by keeping earliest start.
        if let Some(&(top_h, top_start)) = stack.last() {
            if top_h == cur_h {
                // Keep earliest start for this height.
                if top_start > start {
                    stack.pop();
                    stack.push((cur_h, start));
                }
                continue;
            }
        }
        stack.push((cur_h, start));
    }

    // If w == 0, best remains 0, matching the spec.
    print!("{best}\n");
}
